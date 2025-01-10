import type { Node, Token } from "@alc-lisp/wasm";
import { create } from "zustand";

export type PlaygroundStore = {
  tokens: Token[];
  AST: Node | null;
  setTokens: (tokens: Token[]) => void;
  setAST: (ast: Node | null) => void;
};

export const usePlaygroundStore = create<PlaygroundStore>()((set, get) => {
  return {
    AST: null,
    tokens: [],
    setAST(ast) {
      set({ AST: ast });
    },
    setTokens(tokens) {
      set({ tokens: tokens });
    },
  };
});
