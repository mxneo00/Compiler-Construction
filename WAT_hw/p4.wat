(module
  ;; print(i32) is supplied by the host (JS, wasmtime, etc.)
  (import "env" "print" (func $print (param i32)))
  (global $x (mut i32) (i32.const 0))
  (func $main (export "main")
    ;; minilang code starts here
    i32.const 2             ;; stack: [2]
    i32.const 3             ;; stack: [2, 3]
    i32.const 4             ;; stack: [2, 3, 4]
    i32.mul                 ;; stack: [2,12]    (pops 3 and 4, pushes 3 * 4)
    i32.plus                ;; stack: [14]      (pops 2 and 12, pushes 2 + 12)
    global.set $x           ;; stack: []        (pops 14, stores in x)
    global.get $x           ;; stack: [14]      (push 14)
    call $print             ;; stack: []        (pops 20, prints "20")
  )
  (start $main)
)