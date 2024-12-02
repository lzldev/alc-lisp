import { Counter } from "../components/counter";
import { Header } from "../components/Header";
import { App } from "./App";

export function Home() {
  console.log("Hello");

  return (
    <div>
      <Header />
      <App>
        <h1>Home</h1>
        <Counter />
      </App>
    </div>
  );
}
