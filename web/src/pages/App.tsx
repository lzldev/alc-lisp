import init from "@alc-lisp/wasm";
import { useEffect, useState, type PropsWithChildren } from "react";

export function App({ children, ...props }: PropsWithChildren) {
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    (async () => {
      await init();
      setIsLoading(false);
    })();
  }, []);

  if (isLoading) {
    return <>Loading...</>;
  }

  return <>{children}</>;
}
