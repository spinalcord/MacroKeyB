import { G as current_component, z as push, I as fallback, J as attr_class, F as escape_html, K as bind_props, C as pop, M as attr, N as ensure_array_like, O as stringify, P as copy_payload, Q as assign_payload, R as store_get, S as unsubscribe_stores } from "../../chunks/index.js";
import { invoke } from "@tauri-apps/api/core";
import { basicSetup } from "codemirror";
import { EditorView, keymap, placeholder } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { indentWithTab } from "@codemirror/commands";
import { indentUnit, StreamLanguage, HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags } from "@lezer/highlight";
import { snippet, autocompletion, hasNextSnippetField, hasPrevSnippetField } from "@codemirror/autocomplete";
import { lua } from "@codemirror/legacy-modes/mode/lua";
import { w as writable } from "../../chunks/index2.js";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "@tauri-apps/api/menu";
import "@tauri-apps/api/tray";
function onDestroy(fn) {
  var context = (
    /** @type {Component} */
    current_component
  );
  (context.d ??= []).push(fn);
}
function ListBoxItem($$payload, $$props) {
  push();
  let displayText = fallback($$props["displayText"], "(Empty)");
  let labelText = fallback($$props["labelText"], "(Empty)");
  let id = fallback($$props["id"], "123");
  let content = fallback($$props["content"], "text");
  let isSelected = fallback($$props["isSelected"], false);
  let onitem = $$props["onitem"];
  $$payload.out += `<div class="list-item-wrapper svelte-1lpvrn4"><button${attr_class(isSelected ? "list-item-button-selected" : "list-item-button", "svelte-1lpvrn4")}><div class="list-item-content svelte-1lpvrn4"><div class="list-item-text svelte-1lpvrn4">${escape_html(displayText)}</div> <div class="list-item-actions svelte-1lpvrn4"><span class="list-item-tag svelte-1lpvrn4">${escape_html(labelText)}</span></div></div></button></div>`;
  bind_props($$props, {
    displayText,
    labelText,
    id,
    content,
    isSelected,
    onitem
  });
  pop();
}
function ListBox($$payload, $$props) {
  push();
  let filteredItems;
  let Items = fallback($$props["Items"], () => [], true);
  let onitem = $$props["onitem"];
  let nameSearchTerm = "";
  let labelSearchTerm = "";
  filteredItems = Items.filter((item) => {
    item.displayText.toLowerCase().includes(nameSearchTerm.toLowerCase());
    item.assignedKey.toLowerCase().includes(labelSearchTerm.toLowerCase());
    return true;
  });
  $$payload.out += `<div class="list-box svelte-1gmx3qu"><div class="search-container svelte-1gmx3qu"><div class="search-field svelte-1gmx3qu"><label for="nameSearch" class="svelte-1gmx3qu">Name</label> <input id="nameSearch" type="text" placeholder="Search name..."${attr("value", nameSearchTerm)} class="search-input svelte-1gmx3qu"></div> <div class="search-field svelte-1gmx3qu"><label for="labelSearch" class="svelte-1gmx3qu">Label</label> <input id="labelSearch" type="text" placeholder="Search key..."${attr("value", labelSearchTerm)} class="search-input svelte-1gmx3qu"></div></div> <div class="list-content svelte-1gmx3qu">`;
  if (filteredItems.length > 0) {
    $$payload.out += "<!--[-->";
    const each_array = ensure_array_like(filteredItems);
    $$payload.out += `<!--[-->`;
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let item = each_array[$$index];
      ListBoxItem($$payload, {
        onitem: (itemData) => {
          onitem(itemData);
        },
        labelText: item.assignedKey,
        isSelected: item.isSelected,
        displayText: item.displayText,
        content: item.content,
        id: item.id
      });
    }
    $$payload.out += `<!--]-->`;
  } else {
    $$payload.out += "<!--[!-->";
    $$payload.out += `<div class="no-results svelte-1gmx3qu">No items match your search</div>`;
  }
  $$payload.out += `<!--]--></div></div>`;
  bind_props($$props, { Items, onitem });
  pop();
}
function CodeMirror($$payload, $$props) {
  push();
  let classes = fallback($$props["class"], "");
  let value = fallback($$props["value"], "");
  let basic = fallback($$props["basic"], true);
  let lang = fallback($$props["lang"], () => void 0, true);
  let theme = fallback($$props["theme"], () => void 0, true);
  let extensions = fallback($$props["extensions"], () => [], true);
  let useTab = fallback($$props["useTab"], true);
  let tabSize = fallback($$props["tabSize"], 2);
  let styles = fallback($$props["styles"], () => void 0, true);
  let lineWrapping = fallback($$props["lineWrapping"], false);
  let editable = fallback($$props["editable"], true);
  let readonly = fallback($$props["readonly"], false);
  let placeholder$1 = fallback($$props["placeholder"], () => void 0, true);
  let nodebounce = fallback($$props["nodebounce"], false);
  const is_browser = typeof window !== "undefined";
  let view;
  onDestroy(() => view?.destroy());
  function get_base_extensions(basic2, useTab2, tabSize2, lineWrapping2, placeholder2, editable2, readonly2, lang2) {
    const extensions2 = [
      indentUnit.of(" ".repeat(tabSize2)),
      EditorView.editable.of(editable2),
      EditorState.readOnly.of(readonly2)
    ];
    if (basic2) extensions2.push(basicSetup);
    if (useTab2) extensions2.push(keymap.of([indentWithTab]));
    if (placeholder2) extensions2.push(placeholder(placeholder2));
    if (lang2) extensions2.push(lang2);
    if (lineWrapping2) extensions2.push(EditorView.lineWrapping);
    return extensions2;
  }
  function get_theme(theme2, styles2) {
    const extensions2 = [];
    if (styles2) extensions2.push(EditorView.theme(styles2));
    if (theme2) extensions2.push(theme2);
    return extensions2;
  }
  [
    ...get_base_extensions(basic, useTab, tabSize, lineWrapping, placeholder$1, editable, readonly, lang),
    ...get_theme(theme, styles),
    ...extensions
  ];
  if (is_browser) {
    $$payload.out += "<!--[-->";
    $$payload.out += `<div${attr_class(`codemirror-wrapper ${stringify(classes)}`, "svelte-kcx0g9")}></div>`;
  } else {
    $$payload.out += "<!--[!-->";
    $$payload.out += `<div${attr_class(`scm-waiting ${stringify(classes)}`, "svelte-kcx0g9")}><div class="scm-waiting__loading scm-loading svelte-kcx0g9"><div class="scm-loading__spinner svelte-kcx0g9"></div> <p class="scm-loading__text svelte-kcx0g9">Loading editor...</p></div> <pre class="scm-pre cm-editor svelte-kcx0g9">${escape_html(value)}</pre></div>`;
  }
  $$payload.out += `<!--]-->`;
  bind_props($$props, {
    class: classes,
    value,
    basic,
    lang,
    theme,
    extensions,
    useTab,
    tabSize,
    styles,
    lineWrapping,
    editable,
    readonly,
    placeholder: placeholder$1,
    nodebounce
  });
  pop();
}
const luaLanguage = StreamLanguage.define(lua);
const darkTheme = EditorView.theme({
  "&": {
    backgroundColor: "var(--bg-secondary)",
    color: "var(--text-primary)"
  },
  ".cm-content": {
    caretColor: "var(--accent-color)"
  },
  ".cm-cursor": {
    borderLeftWidth: "3px",
    // Increase cursor thickness (default is 1px)
    borderLeftColor: "var(--accent-color)",
    borderLeftStyle: "solid"
  },
  ".cm-activeLine": {
    backgroundColor: "var(--activeLine)"
  },
  ".cm-gutters": {
    backgroundColor: "var(--bg-primary)",
    color: "var(--text-tertiary)",
    border: "none"
  },
  ".cm-activeLineGutter": {
    backgroundColor: "var(--bg-tertiary)"
  },
  ".cm-lineNumbers": {
    color: "var(--text-tertiary)"
  },
  "&.cm-focused .cm-matchingBracket": {
    backgroundColor: "var(--accent-color-transparent)",
    outline: "1px solid var(--accent-color-light)"
  },
  // Uniform selection color
  ".cm-selectionBackground": {
    backgroundColor: "var(--accent-color-transparent) !important",
    // Uniform red as example
    zIndex: 1
  },
  "&.cm-focused .cm-selectionBackground": {
    backgroundColor: "var(--accent-color-transparent) !important",
    // Same color for focused state
    color: "white"
  },
  // Additional selection settings
  ".cm-line ::selection": {
    backgroundColor: "var(--accent-color-transparent) !important"
  },
  "::selection": {
    backgroundColor: "var(--accent-color-transparent) !important"
  },
  // Prevent special highlighting of selected text
  ".cm-selectionMatch": {
    backgroundColor: "transparent !important"
    // Removes the additional highlighting
  },
  ".cm-snippetField": {
    backgroundColor: "var(--accent-color-transparent) !important",
    // VERY noticeable color
    color: "white !important",
    // So that the text remains readable
    outline: "2px dashed red !important",
    // Noticeable border
    borderRadius: "3px"
  },
  ".cm-snippetField-active": {
    backgroundColor: "lime !important",
    // ANOTHER VERY noticeable color
    color: "black !important",
    outline: "2px solid blue !important"
    // Another noticeable border
  }
});
const darkHighlightStyle = HighlightStyle.define([
  { tag: tags.keyword, color: "var(--accent-color)" },
  { tag: tags.operator, color: "var(--text-secondary)" },
  { tag: tags.special(tags.variableName), color: "#FF9800" },
  { tag: tags.comment, color: "var(--text-tertiary)", fontStyle: "italic" },
  { tag: tags.string, color: "#4CAF50" },
  { tag: tags.number, color: "#03A9F4" },
  { tag: tags.function(tags.variableName), color: "#F44336" },
  { tag: tags.definition(tags.variableName), color: "#E91E63" },
  { tag: tags.propertyName, color: "#9C27B0" },
  { tag: tags.bool, color: "#FF5722" }
]);
const darkHighlighting = syntaxHighlighting(darkHighlightStyle);
const luaCompletions = [
  // Keywords with snippet templates
  {
    label: "if",
    type: "keyword",
    apply: snippet("if ${condition} then\n	${}\nend"),
    detail: "If statement"
  },
  {
    label: "ifelse",
    type: "keyword",
    apply: snippet("if ${condition} then\n	${}\nelse\n	\nend"),
    detail: "If-Else statement"
  },
  {
    label: "ifelseif",
    type: "keyword",
    apply: snippet("if ${condition1} then\n	${}\nelseif ${condition2} then\n	\nelse\n	\nend"),
    detail: "If-ElseIf-Else statement"
  },
  {
    label: "for",
    type: "keyword",
    apply: snippet("for ${i}=${1}, ${10} do\n	${}\nend"),
    detail: "Numeric for loop"
  },
  {
    label: "fori",
    type: "keyword",
    apply: snippet("for ${index}, ${value} in ipairs(${myTable}) do\n	${}\nend"),
    detail: "ipairs loop (indexed)"
  },
  {
    label: "forp",
    type: "keyword",
    apply: snippet("for ${key}, ${value} in pairs(${myTable}) do\n	${}\nend"),
    detail: "pairs loop (all keys)"
  },
  {
    label: "while",
    type: "keyword",
    apply: snippet("while ${condition} do\n	${}\nend"),
    detail: "While loop"
  },
  {
    label: "repeat",
    type: "keyword",
    apply: snippet("repeat\n	${}\nuntil ${condition}"),
    detail: "Repeat-Until loop"
  },
  {
    label: "function",
    type: "keyword",
    apply: snippet("function ${myFunction}()\n	${}\nend"),
    detail: "Function definition"
  },
  {
    label: "localfunc",
    type: "keyword",
    apply: snippet("local function ${myFunction}()\n	${}\nend"),
    detail: "Local function definition"
  },
  {
    label: "method",
    type: "keyword",
    apply: snippet("function ${myTable}:${myMethod}()\n	${}\nend"),
    detail: "Method definition"
  },
  {
    label: "local",
    type: "keyword",
    apply: snippet("local ${myVar} = ${myValue}${}"),
    detail: "Local variable"
  },
  {
    label: "table",
    type: "keyword",
    apply: snippet("local ${myTable} = {\n	${}\n}"),
    detail: "Table definition"
  },
  // Regular keywords (no snippet needed for cursor placement)
  { label: "and", type: "keyword" },
  { label: "break", type: "keyword" },
  { label: "do", type: "keyword" },
  { label: "else", type: "keyword" },
  { label: "elseif", type: "keyword" },
  { label: "end", type: "keyword" },
  { label: "false", type: "keyword" },
  { label: "in", type: "keyword" },
  { label: "nil", type: "keyword" },
  { label: "not", type: "keyword" },
  { label: "or", type: "keyword" },
  { label: "return", type: "keyword", apply: snippet("return ${value}") },
  // Optional with field
  { label: "then", type: "keyword" },
  { label: "true", type: "keyword" },
  { label: "until", type: "keyword" },
  {
    label: "print_var",
    type: "snippet",
    apply: snippet('print("${variableName}:", ${variableName})'),
    detail: "Print variable with label"
  },
  {
    label: "error_handle",
    type: "snippet",
    apply: snippet('local status, result = pcall(function()\n	${}\nend)\nif not status then\n	print("Error:", result)\nend'),
    detail: "Error handling with pcall"
  },
  {
    label: "module",
    type: "snippet",
    apply: snippet("local ${MyModule} = {}\n\nfunction ${MyModule}.${myFunction}(${param1, param2})\n	${}\nend\n\nreturn ${MyModule}"),
    detail: "Module pattern"
  },
  {
    label: "class",
    type: "snippet",
    apply: snippet("local ${MyClass} = {}\n${MyClass}.__index = ${MyClass}\n\nfunction ${MyClass}.new(${param1, param2})\n	local self = setmetatable({}, ${MyClass})\n	${}\n	return self\nend\n\nfunction ${MyClass}:${myMethod}()\n	${cursor2}\nend\n\nreturn ${MyClass}"),
    detail: "Simple class pattern"
  },
  // Base functions with descriptions and snippets
  { label: "assert", type: "function", apply: snippet("assert(${v}, ${message})"), info: "assert(v [, message]) - Raises an error if v is false." },
  { label: "collectgarbage", type: "function", apply: snippet("collectgarbage(${opt}, ${arg})"), info: "collectgarbage([opt [, arg]]) - Controls the garbage collector." },
  { label: "dofile", type: "function", apply: snippet("dofile(${filename})"), info: "dofile([filename]) - Executes a Lua chunk from a file." },
  { label: "error", type: "function", apply: snippet("error(${message}, ${level})"), info: "error(message [, level]) - Terminates the last protected function." },
  { label: "getmetatable", type: "function", apply: snippet("getmetatable(${object})"), info: "getmetatable(object) - Returns the metatable of an object." },
  { label: "ipairs", type: "function", apply: snippet("ipairs(${t})"), info: "ipairs(t) - Returns an iterator for numeric keys in a table." },
  { label: "load", type: "function", apply: snippet("load(${chunk}, ${chunkname}, ${mode}, ${env})"), info: "load(chunk [, chunkname [, mode [, env]]]) - Loads a chunk of code." },
  { label: "loadfile", type: "function", apply: snippet("loadfile(${filename}, ${mode}, ${env})"), info: "loadfile([filename [, mode [, env]]]) - Loads a file as a chunk." },
  { label: "next", type: "function", apply: snippet("next(${table}, ${index})"), info: "next(table [, index]) - Returns the next key-value pair in a table." },
  { label: "pairs", type: "function", apply: snippet("pairs(${t})"), info: "pairs(t) - Returns an iterator for all key-value pairs in a table." },
  { label: "pcall", type: "function", apply: snippet("pcall(${f}, ${arg1})"), info: "pcall(f [, arg1, ...]) - Calls function f in protected mode." },
  { label: "print", type: "function", apply: snippet("print(${...})"), info: "print(...) - Prints arguments to stdout." },
  { label: "rawequal", type: "function", apply: snippet("rawequal(${v1}, ${v2})"), info: "rawequal(v1, v2) - Compares v1 and v2 without metamethods." },
  { label: "rawget", type: "function", apply: snippet("rawget(${table}, ${index})"), info: "rawget(table, index) - Gets a value from a table without metamethods." },
  { label: "rawset", type: "function", apply: snippet("rawset(${table}, ${index}, ${value})"), info: "rawset(table, index, value) - Sets a value in a table without metamethods." },
  { label: "require", type: "function", apply: snippet("require(${modname})"), info: "require(modname) - Loads a module." },
  { label: "select", type: "function", apply: snippet("select(${index}, ${...})"), info: "select(index, ...) - Returns arguments after index or total number." },
  { label: "setmetatable", type: "function", apply: snippet("setmetatable(${table}, ${metatable})"), info: "setmetatable(table, metatable) - Sets the metatable for a table." },
  { label: "tonumber", type: "function", apply: snippet("tonumber(${e}, ${base})"), info: "tonumber(e [, base]) - Converts e to a number." },
  { label: "tostring", type: "function", apply: snippet("tostring(${v})"), info: "tostring(v) - Converts v to a string." },
  { label: "type", type: "function", apply: snippet("type(${v})"), info: "type(v) - Returns the type of v as a string." },
  { label: "xpcall", type: "function", apply: snippet("xpcall(${f}, ${msgh}, ${arg1})"), info: "xpcall(f, msgh [, arg1, ...]) - Calls f in protected mode with a message handler." },
  // String library
  { label: "string.byte", type: "function", apply: snippet("string.byte(${s}, ${i}, ${j})"), info: "string.byte(s [, i [, j]]) - Returns numeric codes of characters." },
  { label: "string.char", type: "function", apply: snippet("string.char(${...})"), info: "string.char(...) - Returns a string from numeric character codes." },
  { label: "string.dump", type: "function", apply: snippet("string.dump(${function}, ${strip})"), info: "string.dump(function [, strip]) - Returns binary representation of a function." },
  { label: "string.find", type: "function", apply: snippet("string.find(${s}, ${pattern}, ${init}, ${plain})"), info: "string.find(s, pattern [, init [, plain]]) - Finds pattern in string." },
  { label: "string.format", type: "function", apply: snippet("string.format(${formatstring}, ${...})"), info: "string.format(formatstring, ...) - Returns a formatted string." },
  { label: "string.gmatch", type: "function", apply: snippet("string.gmatch(${s}, ${pattern})"), info: "string.gmatch(s, pattern) - Returns an iterator for pattern matches." },
  { label: "string.gsub", type: "function", apply: snippet("string.gsub(${s}, ${pattern}, ${repl}, ${n})"), info: "string.gsub(s, pattern, repl [, n]) - Replaces pattern occurrences." },
  { label: "string.len", type: "function", apply: snippet("string.len(${s})"), info: "string.len(s) - Returns the length of string s." },
  { label: "string.lower", type: "function", apply: snippet("string.lower(${s})"), info: "string.lower(s) - Converts string to lowercase." },
  { label: "string.match", type: "function", apply: snippet("string.match(${s}, ${pattern}, ${init})"), info: "string.match(s, pattern [, init]) - Finds pattern in string." },
  { label: "string.rep", type: "function", apply: snippet("string.rep(${s}, ${n}, ${sep})"), info: "string.rep(s, n [, sep]) - Repeats a string." },
  { label: "string.reverse", type: "function", apply: snippet("string.reverse(${s})"), info: "string.reverse(s) - Reverses a string." },
  { label: "string.sub", type: "function", apply: snippet("string.sub(${s}, ${i}, ${j})"), info: "string.sub(s, i [, j]) - Returns a substring." },
  { label: "string.upper", type: "function", apply: snippet("string.upper(${s})"), info: "string.upper(s) - Converts string to uppercase." },
  // Table library
  { label: "table.concat", type: "function", apply: snippet("table.concat(${list}, ${sep}, ${i}, ${j})"), info: "table.concat(list [, sep [, i [, j]]]) - Concatenates list elements." },
  { label: "table.insert", type: "function", apply: snippet("table.insert(${list}, ${pos}, ${value})"), info: "table.insert(list, [pos,] value) - Inserts value into list." },
  { label: "table.pack", type: "function", apply: snippet("table.pack(${...})"), info: "table.pack(...) - Packs arguments into a new table." },
  { label: "table.remove", type: "function", apply: snippet("table.remove(${list}, ${pos})"), info: "table.remove(list [, pos]) - Removes element from list." },
  { label: "table.sort", type: "function", apply: snippet("table.sort(${list}, ${comp})"), info: "table.sort(list [, comp]) - Sorts list in-place." },
  { label: "table.unpack", type: "function", apply: snippet("table.unpack(${list}, ${i}, ${j})"), info: "table.unpack(list [, i [, j]]) - Returns elements from list as values." },
  // Math library
  { label: "math.abs", type: "function", apply: snippet("math.abs(${x})"), info: "math.abs(x) - Absolute value." },
  { label: "math.acos", type: "function", apply: snippet("math.acos(${x})"), info: "math.acos(x) - Arc cosine." },
  { label: "math.asin", type: "function", apply: snippet("math.asin(${x})"), info: "math.asin(x) - Arc sine." },
  { label: "math.atan", type: "function", apply: snippet("math.atan(${y}, ${x})"), info: "math.atan(y [, x]) - Arc tangent." },
  { label: "math.ceil", type: "function", apply: snippet("math.ceil(${x})"), info: "math.ceil(x) - Smallest integer >= x." },
  { label: "math.cos", type: "function", apply: snippet("math.cos(${x})"), info: "math.cos(x) - Cosine." },
  { label: "math.deg", type: "function", apply: snippet("math.deg(${x})"), info: "math.deg(x) - Radians to degrees." },
  { label: "math.exp", type: "function", apply: snippet("math.exp(${x})"), info: "math.exp(x) - e^x." },
  { label: "math.floor", type: "function", apply: snippet("math.floor(${x})"), info: "math.floor(x) - Largest integer <= x." },
  { label: "math.fmod", type: "function", apply: snippet("math.fmod(${x}, ${y})"), info: "math.fmod(x,y) - Remainder of x/y." },
  // Hinzugefügt, da oft genutzt
  { label: "math.huge", type: "constant", info: "Value representing infinity." },
  // Konstante, kein Snippet
  { label: "math.log", type: "function", apply: snippet("math.log(${x}, ${base})"), info: "math.log(x [, base]) - Logarithm." },
  { label: "math.max", type: "function", apply: snippet("math.max(${x}, ${...})"), info: "math.max(x, ...) - Maximum value." },
  { label: "math.min", type: "function", apply: snippet("math.min(${x}, ${...})"), info: "math.min(x, ...) - Minimum value." },
  { label: "math.modf", type: "function", apply: snippet("math.modf(${x})"), info: "math.modf(x) - Returns integral and fractional parts of x." },
  // Added
  { label: "math.pi", type: "constant", info: "Mathematical constant pi." },
  // Konstante, kein Snippet
  { label: "math.pow", type: "function", apply: snippet("math.pow(${x}, ${y})"), info: "math.pow(x,y) - x^y (same as x^y)." },
  // Added (x^y is more common)
  { label: "math.rad", type: "function", apply: snippet("math.rad(${x})"), info: "math.rad(x) - Degrees to radians." },
  { label: "math.random", type: "function", apply: snippet("math.random(${m}, ${n})"), info: "math.random([m [, n]]) - Random number." },
  { label: "math.randomseed", type: "function", apply: snippet("math.randomseed(${x})"), info: "math.randomseed(x) - Sets random seed." },
  { label: "math.sin", type: "function", apply: snippet("math.sin(${x})"), info: "math.sin(x) - Sine." },
  { label: "math.sqrt", type: "function", apply: snippet("math.sqrt(${x})"), info: "math.sqrt(x) - Square root." },
  { label: "math.tan", type: "function", apply: snippet("math.tan(${x})"), info: "math.tan(x) - Tangent." },
  { label: "math.tointeger", type: "function", apply: snippet("math.tointeger(${x})"), info: "math.tointeger(x) - Converts x to an integer, or nil if not convertible." },
  // Lua 5.3+
  { label: "math.type", type: "function", apply: snippet("math.type(${x})"), info: "math.type(x) - Returns 'integer', 'float', or nil if x is not a number." },
  // Lua 5.3+
  { label: "math.ult", type: "function", apply: snippet("math.ult(${m}, ${n})"), info: "math.ult(m,n) - Returns boolean indicating if m < n as unsigned integers." }
  // Lua 5.3+
];
const myCompletions = autocompletion({
  override: [
    (context) => {
      const state = context.state;
      if (hasNextSnippetField(state) || hasPrevSnippetField(state)) {
        return null;
      }
      let word = context.matchBefore(/\w*/);
      if (!word || word.from == word.to && !context.explicit)
        return null;
      return {
        from: word.from,
        options: luaCompletions,
        span: /^\w*$/
      };
    }
  ]
});
const cmExtensions = [
  luaLanguage,
  darkTheme,
  darkHighlighting,
  myCompletions,
  EditorView.updateListener.of((update) => {
    if (update.view) {
      update.view;
    }
  })
];
const eventLuaError = writable("");
listen("lua-execution", (event) => {
  eventLuaError.set("");
});
listen("lua-error", (event) => {
  eventLuaError.set(event.payload.message);
});
function _page($$payload, $$props) {
  push();
  var $$store_subs;
  const appWindow = getCurrentWindow();
  appWindow.onCloseRequested((event) => {
    appWindow.hide();
    event.preventDefault();
  });
  let items = [];
  let selectedItemId = null;
  let value = "";
  let saveInterval;
  let assignModeActive = false;
  let statusMessage = "";
  let showStatusPopup = false;
  let statusTimeout = null;
  function showTempStatusMessage(message, duration = 3e3) {
    statusMessage = message;
    showStatusPopup = true;
    if (statusTimeout !== null) {
      clearTimeout(statusTimeout);
    }
    statusTimeout = setTimeout(
      () => {
        if (statusMessage === message) {
          showStatusPopup = false;
          setTimeout(
            () => {
              statusMessage = "";
            },
            300
          );
        }
      },
      duration
    );
  }
  function updateItemInList(id, updates) {
    items = items.map((item) => {
      if (item.id === id) {
        return { ...item, ...updates };
      }
      return item;
    });
  }
  function handleError(error, context, duration = 5e3) {
    console.error(`Error ${context}:`, error);
    const errorMsg = `Error ${context}: ${error}`;
    showTempStatusMessage(errorMsg, duration);
  }
  onDestroy(() => {
    clearInterval(saveInterval);
    if (statusTimeout !== null) {
      clearTimeout(statusTimeout);
    }
  });
  async function itemClicked(targetItem) {
    if (selectedItemId) {
      updateItemInList(selectedItemId, { content: value });
      try {
        await invoke("update_item_content", { id: selectedItemId, content: value });
      } catch (error) {
        handleError(error, "while switching");
      }
    }
    items = items.map((item) => ({
      ...item,
      isSelected: item.id === targetItem.id
    }));
    selectedItemId = targetItem.id;
    value = targetItem.content;
    try {
      await invoke("select_item", { id: targetItem.id });
    } catch (error) {
      handleError(error, "while selecting");
    }
  }
  let $$settled = true;
  let $$inner_payload;
  function $$render_inner($$payload2) {
    $$payload2.out += `<main class="svelte-1ud11i7"><div class="app-container svelte-1ud11i7"><div class="sidebar svelte-1ud11i7"><div class="action-buttons-container svelte-1ud11i7"><button class="action-button primary svelte-1ud11i7">New</button> <button class="action-button primary svelte-1ud11i7"${attr("disabled", true, true)}>Speichern</button> <button class="action-button delete svelte-1ud11i7"${attr("disabled", !selectedItemId, true)}>Rename</button> <button class="action-button delete svelte-1ud11i7"${attr("disabled", !selectedItemId, true)}>Delete</button> <button class="action-button assign-key svelte-1ud11i7">Detect Input Device</button> <button class="action-button assign-key svelte-1ud11i7"${attr("disabled", !selectedItemId || assignModeActive, true)}>Assign Key</button> <button class="action-button cancel svelte-1ud11i7"${attr("disabled", true, true)}>Cancel</button></div> <div class="list-container svelte-1ud11i7">`;
    ListBox($$payload2, { onitem: itemClicked, Items: items });
    $$payload2.out += `<!----></div></div> <div class="main-content svelte-1ud11i7">`;
    CodeMirror($$payload2, {
      class: "codemirror-container",
      extensions: cmExtensions,
      get value() {
        return value;
      },
      set value($$value) {
        value = $$value;
        $$settled = false;
      }
    });
    $$payload2.out += `<!----></div></div> `;
    {
      $$payload2.out += "<!--[!-->";
    }
    $$payload2.out += `<!--]--> `;
    if (statusMessage) {
      $$payload2.out += "<!--[-->";
      $$payload2.out += `<div class="status-popup-container svelte-1ud11i7"><div${attr_class("status-popup svelte-1ud11i7", void 0, {
        "show": showStatusPopup,
        "active": assignModeActive
      })}>${escape_html(statusMessage)}</div></div>`;
    } else {
      $$payload2.out += "<!--[!-->";
    }
    $$payload2.out += `<!--]--> `;
    if (store_get($$store_subs ??= {}, "$eventLuaError", eventLuaError)) {
      $$payload2.out += "<!--[-->";
      $$payload2.out += `<div class="error-container svelte-1ud11i7"><div class="error-box svelte-1ud11i7"><div class="error-header svelte-1ud11i7"><span class="error-title svelte-1ud11i7">Lua Error in: ${escape_html(JSON.parse(store_get($$store_subs ??= {}, "$eventLuaError", eventLuaError)).itemName)}</span></div> <div class="error-content svelte-1ud11i7"><p class="error-message svelte-1ud11i7">${escape_html(JSON.parse(store_get($$store_subs ??= {}, "$eventLuaError", eventLuaError)).error)}</p> <p class="error-timestamp svelte-1ud11i7">${escape_html(JSON.parse(store_get($$store_subs ??= {}, "$eventLuaError", eventLuaError)).timestamp)}</p></div></div></div>`;
    } else {
      $$payload2.out += "<!--[!-->";
    }
    $$payload2.out += `<!--]--></main>`;
  }
  do {
    $$settled = true;
    $$inner_payload = copy_payload($$payload);
    $$render_inner($$inner_payload);
  } while (!$$settled);
  assign_payload($$payload, $$inner_payload);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
export {
  _page as default
};
