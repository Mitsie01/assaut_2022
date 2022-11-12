# Assaut 2022

## Kamelenrace

### Rasppi
Rasppi contains program files for the main Raspberry Pi Model 3B, which is the main controller for the project.

It contains:
- audio/
    Audio files which will be played during operation. 
    - fx/       | Sound effects for race start/finish.
    - idle/     | Ambient music when the machine is in idle status.
    - racing /  | Suspenseful music that will be played during racing sessions.

- main/
    Contains the rust project files (source code and cargo files)

    The code will run idle music until a starting signal is triggered. Then it will play an effect sound to clarify the start of a racing session.
    After this the racing music will begin. The speed of the racing sound is based on the race progress. 
    When the race is finished and ending sound effect will be played based on the winning lane. Then the program will return to idle.
