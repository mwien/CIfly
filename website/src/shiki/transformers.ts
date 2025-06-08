import { h } from "hastscript";
import type { Element, Root, Text } from "hast";

export function addCopyButton() {
  return {
    name: "add-copy-button",
    root(root: Root) {
      if (root.children.length === 0) {
        return root; // Return unchanged if no children
      }
      const button: Element = h(
        "button",
        {
          class:
            "absolute top-0 right-0 p-1.5 md:p-2.5 bg-transparent hover:text-gray-500 cursor-pointer",
        },
        h(
          "svg",
          {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            viewBox: "0 0 24 24",
            "stroke-width": "1.5",
            stroke: "currentColor",
            class: "size-5 md:size-6",
          },
          h("path", {
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            d: "M8.25 7.5V6.108c0-1.135.845-2.098 1.976-2.192.373-.03.748-.057 1.123-.08M15.75 18H18a2.25 2.25 0 0 0 2.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 0 0-1.123-.08M15.75 18.75v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5A3.375 3.375 0 0 0 6.375 7.5H5.25m11.9-3.664A2.251 2.251 0 0 0 15 2.25h-1.5a2.251 2.251 0 0 0-2.15 1.586m5.8 0c.065.21.1.433.1.664v.75h-6V4.5c0-.231.035-.454.1-.664M6.75 7.5H4.875c-.621 0-1.125.504-1.125 1.125v12c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V16.5a9 9 0 0 0-9-9Z",
          }),
        ),
      );
      // assume first child contains code
      const childElement = root.children[0] as Element;
      const wrapper: Element = h(
        "div",
        {
          class: "relative",
        },
        childElement,
      );
      childElement.children.push(button);
      return {
        type: "root",
        children: [wrapper],
      };
    },
  };
}

export function addHeader(title: String, truncate: Boolean) {
  return {
    name: "add-header",
    root(root: Root) {
      if (root.children.length === 0) {
        return root; // Return unchanged if no children
      }

      // assume first child contains code
      const childElement = root.children[0] as Element;

      const headerDiv: Element = h(
        "div",
        {
          class:
            "bg-gray-50 font-mono border-b border-gray-200 px-2 py-2 flex items-center h-10.25",
        },
        h(
          "span",
          {
            class: "max-w-1/2 md:max-w-3/4" + (truncate ? " truncate" : ""),
          },
          [{ type: "text", value: title } as Text],
        ),
      );

      const wrapper: Element = h(
        "div",
        {
          class:
            "not-prose rounded-lg shadow-sm border border-gray-200 overflow-hidden",
        },
        [headerDiv, childElement],
      );

      return {
        type: "root",
        children: [wrapper],
      };
    },
  };
}
