# term-bash
Launch MSYS2 bash from integrated terminals in editors/IDEs such as Visual Studio Code and IntelliJ IDEA.

Currently executes `C:/msys64/usr/bin/bash.exe` with `--login` and `-i` arguments.

Also sets the following environment variables:
* `CHERE_INVOKING=1` ensures that bash starts in project directory.
* `MSYS2_PATH_TYPE=inherit` exports current PATH to bash.
* `PS1=$` minimal PS1 prompt suitable for intergrated terminals.

Exit code of bash is ignored to prevent alerts in Visual Studio Code.
