pub const BASH: &str = "function ws {
    while read -r line; do
        if [[ \"$line\" == \"RUN>*\" ]]; then
            source <(echo \"${line:4}\");
        else
            echo \"$line\";
        fi;
    done < <( workspace \"$@\" );
}";

pub const POWERSHELL: &str = "function ws {
    workspace $args | % {
        if ($_ -match \"^RUN>\") {
            . ([scriptblock]::Create($_.Substring(4)))
        } else {
            Write-Output $_
        }
    }
}";

pub const CMD: &str = "@ECHO off
FOR /F \"tokens=* delims=\" %%G IN ('workspace %*') DO (
    CALL :subroutine \"%%G\"
)
GOTO :EOF

:subroutine
    SET \"temp=%~1\"
    IF \"%temp:~0,4%\" == \"RUN>\" (
        CALL %temp:~4%
    ) ELSE (
        ECHO %~1
    )
    GOTO :EOF";
