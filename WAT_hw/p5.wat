(module
  ;; print(i32) is supplied by the host (JS, wasmtime, etc.)
  (import "env" "print" (func $print (param i32)))
  (global $x (mut i32) (i32.const 0))
  (func $main (export "main")
    ;; minilang code starts here
    i32.const 0             ;; stack: [0]
    global.set $x           ;; stack: []        (pops 0, stores in x)
    i32.const 5             ;; stack: [5]
    i32.const 3             ;; stack: [5, 3]
    i32.gt_s                ;; stack: [1]       (pops 5 and 3, push 1 (5>3 = true))
    if                      ;; stack: []        (pops 1)
        i32.const 100       ;; stack: [100]     
        global.set $x       ;; stack: []        (pops 100, stores in x)
    else
        i32.const 200       ;; stack: [200]     
        global.set $x       ;; stack: []        (pops 200, stores in x)
    end
    global.get $x           ;; stack: [100]     (push 100 since true)
    call $print             ;; stack: []        (pops 100, prints "100")
  )
  (start $main)
)