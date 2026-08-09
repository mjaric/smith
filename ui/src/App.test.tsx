import { expect, test } from "vitest";
import { render, screen } from "@testing-library/react";
import App from "./App";

test("renders the Smith shell", () => {
  render(<App />);
  expect(screen.getByRole("heading", { level: 1 })).toHaveTextContent("Smith");
});
