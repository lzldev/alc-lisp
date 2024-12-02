import { add2 } from "alc-lisp-wasm";
import { useState } from "react";

export function Counter() {
  const [count, setCount] = useState(0);

  return (
    <button onClick={() => setCount((count) => add2(count, 1))}>
      count is {count}
    </button>
  );
}
