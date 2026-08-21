import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import json
import os


class TimetableEditor:
    def __init__(self, root):
        self.root = root
        self.root.title("Timetable Editor V0.2")
        self.root.geometry("900x600")

        self.data = []
        self.current_index = None
        self.entries = {}

        self.create_ui()
        self.show_warning()

    def show_warning(self):
        warning = tk.Toplevel(self.root)

        warning.title("Important")
        warning.geometry("500x350")

        warning.transient(self.root)
        warning.grab_set()

        # Prevent the window being closed without acknowledging
        warning.protocol(
            "WM_DELETE_WINDOW",
            warning.destroy
        )

        frame = ttk.Frame(warning, padding=25)
        frame.pack(fill="both", expand=True)

        ttk.Label(
            frame,
            text="⚠  Important",
            font=("TkDefaultFont", 16, "bold")
        ).pack(anchor="w", pady=(0, 20))

        message = (
            "This tool is currently provided without "
            "timetable validation logic.\n\n"

            "The editor will allow you to enter values "
            "that may be invalid or incompatible with "
            "the game.\n\n"

            "Please ensure that you understand the "
            "timetable format and check your timetable "
            "carefully before using it in-game.\n\n"

            "Incorrect values may result in an invalid "
            "timetable.\n\n"

            "This is not an official tool, and it may cause"
            "issues with your game. Please report any issues"
            "with custom timetables to me, so I can validate"
            "and identify if the timetable editor is the cause"
        )

        ttk.Label(
            frame,
            text=message,
            wraplength=440,
            justify="left"
        ).pack(
            anchor="w",
            fill="x"
        )

        ttk.Button(
            frame,
            text="I Understand",
            command=warning.destroy
        ).pack(
            side="bottom",
            pady=(20, 0)
        )

        # Centre the warning window
        warning.update_idletasks()

        warning.wait_window()

    def create_ui(self):

        menu_bar = tk.Menu(self.root)

        # File menu
        file_menu = tk.Menu(
            menu_bar,
            tearoff=0
        )

        file_menu.add_command(
            label="New Timetable",
            command=self.new_timetable
        )

        file_menu.add_separator()

        file_menu.add_command(
            label="Load JSON",
            command=self.load_json
        )

        file_menu.add_command(
            label="Save JSON",
            command=self.save_json
        )

        menu_bar.add_cascade(
            label="File",
            menu=file_menu
        )

        # Edit menu
        edit_menu = tk.Menu(
            menu_bar,
            tearoff=0
        )

        edit_menu.add_command(
            label="Add Entry",
            command=self.add_entry
        )

        edit_menu.add_command(
            label="Delete Entry",
            command=self.delete_entry
        )

        menu_bar.add_cascade(
            label="Edit",
            menu=edit_menu
        )

        self.root.config(
            menu=menu_bar
        )

        main = ttk.Frame(self.root)
        main.pack(
            fill="both",
            expand=True,
            padx=10,
            pady=10
        )

        left_frame = ttk.Frame(main)
        left_frame.pack(
            side="left",
            fill="y"
        )

        ttk.Label(
            left_frame,
            text="Timetable Entries",
            font=("TkDefaultFont", 11, "bold")
        ).pack(
            pady=(0, 5)
        )

        list_container = ttk.Frame(left_frame)
        list_container.pack(
            fill="both",
            expand=True
        )

        self.id_list = tk.Listbox(
            list_container,
            width=15,
            exportselection=False
        )

        scrollbar = ttk.Scrollbar(
            list_container,
            orient="vertical",
            command=self.id_list.yview
        )

        self.id_list.configure(
            yscrollcommand=scrollbar.set
        )

        self.id_list.pack(
            side="left",
            fill="y"
        )

        scrollbar.pack(
            side="right",
            fill="y"
        )

        self.id_list.bind(
            "<<ListboxSelect>>",
            self.on_id_selected
        )

        right_frame = ttk.Frame(main)
        right_frame.pack(
            side="left",
            fill="both",
            expand=True,
            padx=(20, 0)
        )

        self.title_label = ttk.Label(
            right_frame,
            text="No timetable loaded",
            font=("TkDefaultFont", 14, "bold")
        )

        self.title_label.pack(
            anchor="w",
            pady=(0, 15)
        )

        self.canvas = tk.Canvas(
            right_frame,
            highlightthickness=0
        )

        form_scrollbar = ttk.Scrollbar(
            right_frame,
            orient="vertical",
            command=self.canvas.yview
        )

        self.form_frame = ttk.Frame(
            self.canvas
        )

        self.canvas_window = self.canvas.create_window(
            (0, 0),
            window=self.form_frame,
            anchor="nw"
        )

        self.canvas.configure(
            yscrollcommand=form_scrollbar.set
        )

        self.canvas.pack(
            side="left",
            fill="both",
            expand=True
        )

        form_scrollbar.pack(
            side="right",
            fill="y"
        )

        self.form_frame.bind(
            "<Configure>",
            lambda e: self.canvas.configure(
                scrollregion=self.canvas.bbox("all")
            )
        )

        self.canvas.bind(
            "<Configure>",
            self.on_canvas_configure
        )

    def on_canvas_configure(self, event):
        self.canvas.itemconfig(
            self.canvas_window,
            width=event.width
        )

    def new_timetable(self):

        if self.data:

            confirm = messagebox.askyesno(
                "New Timetable",
                "Starting a new timetable will discard "
                "the current timetable.\n\n"
                "Are you sure you want to continue?"
            )

            if not confirm:
                return

        self.data = []
        self.current_index = None
        self.entries.clear()

        self.id_list.delete(
            0,
            tk.END
        )

        for widget in self.form_frame.winfo_children():
            widget.destroy()

        self.title_label.config(
            text="New timetable"
        )

        self.canvas.yview_moveto(0)

    def load_json(self):

        filename = filedialog.askopenfilename(
            title="Open timetable",
            initialdir=os.path.join(os.environ["APPDATA"], "WhitePawGames", "timetables"),
            filetypes=[
                ("JSON files", "*.json"),
                ("All files", "*.*")
            ]
        )

        if not filename:
            return

        try:

            with open(
                filename,
                "r",
                encoding="utf-8"
            ) as f:

                self.data = json.load(f)

        except Exception as e:

            messagebox.showerror(
                "Error",
                f"Could not load JSON:\n\n{e}"
            )

            return

        if not isinstance(
            self.data,
            list
        ):

            messagebox.showerror(
                "Error",
                "The JSON file must contain a list "
                "of timetable entries."
            )

            self.data = []

            return

        self.current_index = None

        self.populate_id_list()

    def populate_id_list(self):

        self.id_list.delete(
            0,
            tk.END
        )

        for entry in self.data:

            self.id_list.insert(
                tk.END,
                entry.get(
                    "id",
                    "Unknown"
                )
            )

        if self.data:

            self.id_list.selection_set(0)

            self.id_list.event_generate(
                "<<ListboxSelect>>"
            )

        else:

            self.title_label.config(
                text="No entries"
            )

            for widget in self.form_frame.winfo_children():
                widget.destroy()

            self.entries.clear()

    def on_id_selected(self, event=None):

        selection = self.id_list.curselection()

        if not selection:
            return

        index = selection[0]

        if self.current_index is not None:

            self.save_current(
                silent=True
            )

        self.current_index = index

        self.show_entry(
            self.data[index]
        )

    def show_entry(self, entry):

        for widget in self.form_frame.winfo_children():
            widget.destroy()

        self.entries.clear()

        entry_id = entry.get(
            "id",
            "Unknown"
        )

        self.title_label.config(
            text=f"Entry: {entry_id}"
        )

        row = 0

        ttk.Label(
            self.form_frame,
            text="ID:"
        ).grid(
            row=row,
            column=0,
            sticky="nw",
            padx=(0, 15),
            pady=6
        )

        id_var = tk.StringVar(
            value=str(entry_id)
        )

        id_widget = ttk.Entry(
            self.form_frame,
            textvariable=id_var,
            width=40
        )

        id_widget.grid(
            row=row,
            column=1,
            sticky="ew",
            pady=4
        )

        self.entries["id"] = (
            "value",
            id_var
        )

        row += 1

        path = str(
            entry.get(
                "path",
                "DownMain"
            )
        )

        if path.startswith("Up"):
            direction = "Up"
        else:
            direction = "Down"

        if direction == "Down":
            if entry.get("destination") == "Goton":
                route = "Branch"
            else:
                route = "Main"
        else:
            if path.endswith("Branch"):
                route = "Branch"
            else:
                route = "Main"

        ttk.Label(
            self.form_frame,
            text="Direction:"
        ).grid(
            row=row,
            column=0,
            sticky="nw",
            padx=(0, 15),
            pady=6
        )

        direction_var = tk.StringVar(
            value=direction
        )

        direction_widget = ttk.Combobox(
            self.form_frame,
            textvariable=direction_var,
            values=[
                "Down",
                "Up"
            ],
            state="readonly",
            width=37
        )

        direction_widget.grid(
            row=row,
            column=1,
            sticky="ew",
            pady=4
        )

        self.entries["direction"] = (
            "value",
            direction_var
        )

        row += 1

        ttk.Label(
            self.form_frame,
            text="Route:"
        ).grid(
            row=row,
            column=0,
            sticky="nw",
            padx=(0, 15),
            pady=6
        )

        route_var = tk.StringVar(
            value=route
        )

        route_widget = ttk.Combobox(
            self.form_frame,
            textvariable=route_var,
            values=[
                "Main",
                "Branch"
            ],
            state="readonly",
            width=37
        )

        route_widget.grid(
            row=row,
            column=1,
            sticky="ew",
            pady=4
        )

        self.entries["route"] = (
            "value",
            route_var
        )


        path_var = tk.StringVar()

        self.entries["path"] = (
            "value",
            path_var
        )

        row += 1

        ttk.Label(
            self.form_frame,
            text="From Instrument:"
        ).grid(
            row=row,
            column=0,
            sticky="nw",
            padx=(0, 15),
            pady=6
        )

        from_var = tk.StringVar()

        from_widget = ttk.Entry(
            self.form_frame,
            textvariable=from_var,
            state="readonly",
            width=40
        )

        from_widget.grid(
            row=row,
            column=1,
            sticky="ew",
            pady=4
        )

        self.entries["from_instrument"] = (
            "value",
            from_var
        )

        row += 1

        ttk.Label(
            self.form_frame,
            text="Destination:"
        ).grid(
            row=row,
            column=0,
            sticky="nw",
            padx=(0, 15),
            pady=6
        )

        destination_var = tk.StringVar()

        destination_widget = ttk.Entry(
            self.form_frame,
            textvariable=destination_var,
            state="readonly",
            width=40
        )

        destination_widget.grid(
            row=row,
            column=1,
            sticky="ew",
            pady=4
        )

        self.entries["destination"] = (
            "value",
            destination_var
        )

        row += 1

        ttk.Label(
            self.form_frame,
            text="Arrival Time:"
        ).grid(
            row=row,
            column=0,
            sticky="nw",
            padx=(0, 15),
            pady=6
        )

        arrival_var = tk.StringVar(
            value=str(
                entry.get(
                    "arrival_time",
                    ""
                )
            )
        )

        arrival_widget = ttk.Entry(
            self.form_frame,
            textvariable=arrival_var,
            width=40
        )

        arrival_widget.grid(
            row=row,
            column=1,
            sticky="ew",
            pady=4
        )

        self.entries["arrival_time"] = (
            "value",
            arrival_var
        )

        row += 1

        ttk.Label(
            self.form_frame,
            text="Departure Time:"
        ).grid(
            row=row,
            column=0,
            sticky="nw",
            padx=(0, 15),
            pady=6
        )

        departure_var = tk.StringVar(
            value=str(
                entry.get(
                    "departure_time",
                    ""
                )
            )
        )

        departure_widget = ttk.Entry(
            self.form_frame,
            textvariable=departure_var,
            width=40
        )

        departure_widget.grid(
            row=row,
            column=1,
            sticky="ew",
            pady=4
        )

        self.entries["departure_time"] = (
            "value",
            departure_var
        )

        row += 1

        def update_path_and_locations(*args):

            selected_direction = direction_var.get()
            selected_route = route_var.get()

            # Down trains must spawn on DownMain,
            # even when their destination is the branch.
            if selected_direction == "Down":
                new_path = "DownMain"
            else:
                new_path = (
                    "Up" +
                    selected_route
                )

            path_var.set(
                new_path
            )

            if selected_direction == "Down":

                from_location = "Chippinhall"

                if selected_route == "Main":
                    destination = "Doortown"
                else:
                    destination = "Goton"

            else:

                destination = "Chippinhall"

                if selected_route == "Main":
                    from_location = "Doortown"
                else:
                    from_location = "Goton"

            from_var.set(
                from_location
            )

            destination_var.set(
                destination
            )

        direction_var.trace_add(
            "write",
            update_path_and_locations
        )

        route_var.trace_add(
            "write",
            update_path_and_locations
        )

        update_path_and_locations()

        for key, value in entry.items():

            if key in (
                "id",
                "path",
                "progress",
                "speed",
                "arrival_time",
                "departure_time",
                "destination",
                "destination_progress",
                "from_instrument",
                "bell_code"
            ):
                continue

            ttk.Label(
                self.form_frame,
                text=self.format_label(key) + ":"
            ).grid(
                row=row,
                column=0,
                sticky="nw",
                padx=(0, 15),
                pady=6
            )

            if isinstance(value, list):

                list_frame = ttk.Frame(
                    self.form_frame
                )

                list_frame.grid(
                    row=row,
                    column=1,
                    sticky="ew",
                    pady=4
                )

                variables = []

                for item in value:

                    var = tk.StringVar(
                        value=str(item)
                    )

                    entry_widget = ttk.Entry(
                        list_frame,
                        textvariable=var,
                        width=12
                    )

                    entry_widget.pack(
                        side="left",
                        padx=(0, 5)
                    )

                    variables.append(
                        var
                    )

                self.entries[key] = (
                    "list",
                    variables
                )

            else:

                var = tk.StringVar(
                    value=str(value)
                )

                entry_widget = ttk.Entry(
                    self.form_frame,
                    textvariable=var,
                    width=40
                )

                entry_widget.grid(
                    row=row,
                    column=1,
                    sticky="ew",
                    pady=4
                )

                self.entries[key] = (
                    "value",
                    var
                )

            row += 1

        self.form_frame.columnconfigure(
            1,
            weight=1
        )

        self.canvas.configure(
            scrollregion=self.canvas.bbox("all")
        )

    def save_current(self, silent=False):

        if self.current_index is None:
            return

        entry = self.data[
            self.current_index
        ]

        old_id = entry.get(
            "id",
            ""
        )

        for key, field_data in self.entries.items():

            field_type, variables = field_data

            if field_type == "value":

                value = variables.get()

                if key in (
                    "direction",
                    "route",
                    "path",
                    "from_instrument",
                    "destination"
                ):
                    continue

                if key != "id":

                    original = entry.get(
                        key
                    )

                    if isinstance(
                        original,
                        float
                    ):

                        try:
                            value = float(value)
                        except ValueError:
                            pass

                    elif isinstance(
                        original,
                        int
                    ):

                        try:
                            value = int(value)
                        except ValueError:
                            pass

                entry[key] = value

            elif field_type == "list":

                original = entry.get(
                    key,
                    []
                )

                new_values = []

                for variable, original_value in zip(
                    variables,
                    original
                ):

                    value = variable.get()

                    if isinstance(
                        original_value,
                        int
                    ):

                        try:
                            value = int(value)
                        except ValueError:
                            pass

                    elif isinstance(
                        original_value,
                        float
                    ):

                        try:
                            value = float(value)
                        except ValueError:
                            pass

                    new_values.append(
                        value
                    )

                entry[key] = new_values

        direction = self.entries[
            "direction"
        ][1].get()

        route = self.entries[
            "route"
        ][1].get()

        if direction == "Down":

            entry["path"] = "DownMain"

        else:

            entry["path"] = (
                "Up" +
                route
            )

        if direction == "Down":

            entry["from_instrument"] = "Chippinhall"

            if route == "Main":
                entry["destination"] = "Doortown"

            else:
                entry["destination"] = "Goton"

        else:

            entry["destination"] = "Chippinhall"

            if route == "Main":
                entry["from_instrument"] = "Doortown"

            else:
                entry["from_instrument"] = "Goton"

        entry["progress"] = 0.0
        entry["speed"] = 0.2
        entry["destination_progress"] = "end"
        entry["bell_code"] = [3, 1]


        new_id = entry.get(
            "id",
            ""
        )

        if old_id != new_id:

            self.id_list.delete(
                self.current_index
            )

            self.id_list.insert(
                self.current_index,
                new_id
            )

            self.id_list.selection_set(
                self.current_index
            )

            self.title_label.config(
                text=f"Entry: {new_id}"
            )

    def add_entry(self):

        if self.current_index is not None:

            self.save_current(
                silent=True
            )

        number = 1

        while True:

            new_id = f"1A01"

            if not any(
                entry.get("id") == new_id
                for entry in self.data
            ):
                break

            number += 1

        new_entry = {
            "id": new_id,
            "path": "DownMain",
            "progress": 0.0,
            "speed": 0.2,
            "arrival_time": "",
            "departure_time": "",
            "destination": "Doortown",
            "destination_progress": "end",
            "from_instrument": "Chippinhall",
            "bell_code": [3, 1]
        }

        self.data.append(
            new_entry
        )

        new_index = len(
            self.data
        ) - 1

        self.id_list.insert(
            tk.END,
            new_id
        )

        self.id_list.selection_clear(
            0,
            tk.END
        )

        self.id_list.selection_set(
            new_index
        )

        self.id_list.see(
            new_index
        )

        self.current_index = new_index

        self.show_entry(
            new_entry
        )

    def delete_entry(self):

        if self.current_index is None:
            return

        entry = self.data[
            self.current_index
        ]

        entry_id = entry.get(
            "id",
            "Unknown"
        )

        confirm = messagebox.askyesno(
            "Delete Entry",
            f"Are you sure you want to delete {entry_id}?"
        )

        if not confirm:
            return

        index = self.current_index

        del self.data[index]

        self.id_list.delete(
            index
        )

        self.current_index = None

        if self.data:

            if index >= len(
                self.data
            ):

                index = len(
                    self.data
                ) - 1

            self.id_list.selection_set(
                index
            )

            self.id_list.see(
                index
            )

            self.current_index = index

            self.show_entry(
                self.data[index]
            )

        else:

            self.title_label.config(
                text="No entries"
            )

            for widget in self.form_frame.winfo_children():
                widget.destroy()

            self.entries.clear()

    def save_json(self):

        self.save_current(
            silent=True
        )

        filename = filedialog.asksaveasfilename(
            title="Save timetable",
            defaultextension=".json",
            filetypes=[
                ("JSON files", "*.json"),
                ("All files", "*.*")
            ]
        )

        if not filename:
            return

        try:

            with open(
                filename,
                "w",
                encoding="utf-8"
            ) as f:

                json.dump(
                    self.data,
                    f,
                    indent=2
                )

            messagebox.showinfo(
                "Saved",
                "Timetable saved successfully."
            )

        except Exception as e:

            messagebox.showerror(
                "Error",
                f"Could not save JSON:\n\n{e}"
            )

    @staticmethod
    def format_label(key):
        return key.replace("_"," ").title()


root = tk.Tk()

app = TimetableEditor(
    root
)

root.mainloop()