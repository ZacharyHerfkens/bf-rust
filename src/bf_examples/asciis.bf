print all printable chars; start at 32; end at 12
>                   ; move to counter
>++++++++++         ; init mult to 10
[<+++++++++>-]<     ; counter contains 90
++++                ; counter contains 94 (num printed chars)
>+++++              ; init mult to 5
[<<+++ +++>>-]<<    ; byte contains 30
++                  ; byte contains 32
; begin print loop
.>[-<+.>]           ; print 94 bytes starting from 32
<                   ; move to byte
++++++++++.         ; print new line
