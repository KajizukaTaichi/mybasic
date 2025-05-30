REM NUMBER TO SOLVE
10 LET NUMBER = 5
20 CALL FACTORIAL
30 EXIT PROGRAM

REM CALCULATE FACTORIAL OF THE NUMBER
SUB FACTORIAL
    REM INITIALIZE VARIABLES
    LET COUNT = 0
    LET RESULT = 1

    REM LOOP TO CALCULATE FACTORIAL
    WHILE TRUE
        REM CHECK IF IT COMES TO END OF LOOP
        IF COUNT = NUMBER
            REM BREAK FROM LOOP
            EXIT WHILE
        END IF

        REM INCREMENT COUNTER
        LET COUNT = COUNT + 1

        REM MULTIPLY RESULT BY COUNT
        LET RESULT = RESULT * COUNT
    END WHILE

    REM RESET COUNTER
    LET COUNT = 0
END SUB
