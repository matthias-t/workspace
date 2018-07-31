pub const BASH: &str = r#"function ws {
    while read -r line; do
        if [[ "$line" == RUN\>* ]]; then
            eval "${line:4}";
        else
            echo "$line";
        fi;
    done < <( workspace "$@" );
}"#;

pub const FISH: &str = r#"function ws
    workspace $argv | while read line
        if set command (string replace "RUN>" "" $line)
            eval $command
        else
            echo $line
        end
    end
end"#;

pub const POWERSHELL: &str = r#"function ws {
    workspace $args | % {
        if ($_ -match "^RUN>") {
            . ([scriptblock]::Create($_.Substring(4)))
        } else {
            Write-Output $_
        }
    }
}"#;

pub const CMD: &str = r#"@ECHO off
FOR /F "tokens=* delims=" %%G IN ('workspace %*') DO (
    CALL :subroutine "%%G"
)
GOTO :EOF

:subroutine
    SET "temp=%~1"
    IF "%temp:~0,4%" == "RUN>" (
        CALL %temp:~4%
    ) ELSE (
        ECHO %~1
    )
    GOTO :EOF"#;
