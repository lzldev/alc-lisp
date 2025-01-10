import type { Node, Token } from "@alc-lisp/wasm";
import { create } from "zustand";

export type PlaygroundStore = {
  tokens: Token[];
  messages: string[];
  AST: Node | null;
  setMessages: (messages: string[]) => void;
  addMessages: (messages: string[]) => void;
  setTokens: (tokens: Token[]) => void;
  setAST: (ast: Node | null) => void;
};

export const usePlaygroundStore = create<PlaygroundStore>()((set, get) => {
  return {
    AST: null,
    messages: [],
    tokens: [],
    setMessages(messages) {
      set({ messages: messages });
    },
    addMessages(messages) {
      set((s) => ({ messages: [...s.messages, ...messages] }));
    },
    setAST(ast) {
      set({ AST: ast });
    },
    setTokens(tokens) {
      set({ tokens: tokens });
    },
  };
});
