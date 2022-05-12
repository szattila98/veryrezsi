import "@testing-library/jest-dom";
import Login from "../routes/login.svelte";
import { render } from "@testing-library/svelte";

describe("Test if Jest is working", () => {
	test("Welcome", () => {
		const { getByText } = render(Login);
		expect(getByText("Welcome to SvelteKit")).toBeInTheDocument();
	  });
  });