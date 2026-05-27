(module
  ;; print(i32) is supplied by the host (JS, wasmtime, etc.)
  (import "env" "print" (func $print (param i32)))
  (global $x (mut i32) (i32.const 0))
  (func $main (export "main")
    ;; minilang code starts here
    i32.const 42                ;; stack: [42]
    call $print                 ;; stack: []    (pops 42, prints "42")
  )
  (start $main)
)