import type { Route } from "./+types/home";
import { Welcome } from "../welcome/welcome";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Play Chess" },
    { name: "description", content: "Play against a chess engine!" },
  ];
}

export default function Home() {
  return <Welcome />;
}
