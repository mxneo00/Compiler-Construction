(module
  ;; print(i32) is supplied by the host (JS, wasmtime, etc.)
  (import "env" "print" (func $print (param i32)))
  (global $x (mut i32) (i32.const 0))
  (func $main (export "main")
    ;; minilang code starts here
    i32.const 10            ;; stack: [10]
    global.set $x           ;; stack: []            (pops 10, stores in x)
    global.get $x           ;; stack: [10]          (push 10)
    call $print             ;; stack: []            (prints "10")
  )
  (start $main)
)