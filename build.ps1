param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$Version,

    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$DxArguments
)

$env:ASTTE_VERSION = $Version
& dx build @DxArguments
exit $LASTEXITCODE