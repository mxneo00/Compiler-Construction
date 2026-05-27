(module
  ;; print(i32) is supplied by the host (JS, wasmtime, etc.)
  (import "env" "print" (func $print (param i32)))
  (global $x (mut i32) (i32.const 0))
  (func $main (export "main")
    ;; minilang code starts here
    i32.const 7                 ;; stack: [7]
    i32.const 3                 ;; stack: [7, 3]
    i32.add                    ;; stack: [10]      (pops 7 and 3, pushes 7 + 3)
    call $print                 ;; stack: []        (pops 10, prints "10")
  )
  (start $main)
)