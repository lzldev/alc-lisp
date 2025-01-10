import init, {
  add_print_callback,
  remove_print_callback,
  type Object,
} from "@alc-lisp/wasm";

import * as AlcLisp from "@alc-lisp/wasm";
import { useEffect, useState } from "react";

export function useAlcLispInit() {
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    (async () => {
      if (!window) return;
      await init();
      setIsLoading(false);
    })();
  }, []);

  return {
    isLoading,
  };
}

export type PrintCallback = (...objects: Object[]) => void;

export function usePrintCallback(callback: PrintCallback) {
  const { isLoading } = useAlcLispInit();

  useEffect(() => {
    if (isLoading) return;
    if (!add_print_callback) return;

    add_print_callback(callback);

    return () => {
      remove_print_callback(callback);
    };
  }, [isLoading]);
}

const FakeFns = Object.keys(AlcLisp).reduce((acc, key) => {
  acc[key] = () => {};
  return acc;
}, {} as any);

export function useAlcLispRun(): typeof AlcLisp {
  const { isLoading } = useAlcLispInit();
  console.log("loading...");
  if (isLoading) return FakeFns;
  console.log("loaded...");

  return AlcLisp;
}
