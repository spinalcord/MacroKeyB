import { EditorView } from "@codemirror/view";
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags } from "@lezer/highlight";
import { autocompletion, startCompletion, snippet, hasNextSnippetField, hasPrevSnippetField } from "@codemirror/autocomplete";
import { StreamLanguage } from "@codemirror/language";
import { lua } from "@codemirror/legacy-modes/mode/lua";
import { undo, redo, selectAll } from '@codemirror/commands';

// Define Lua language
export const luaLanguage = StreamLanguage.define(lua);

// Define Custom Theme for CodeMirror
export const darkTheme = EditorView.theme({
    "&": {
        backgroundColor: "var(--bg-secondary)",
        color: "var(--text-primary)",
    },
    ".cm-content": {
        caretColor: "var(--accent-color)"
    },
    ".cm-cursor": {
        borderLeftWidth: "3px", // Increase cursor thickness (default is 1px)
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
        backgroundColor: "var(--accent-color-transparent) !important", // Uniform red as example
        zIndex: 1
    },
    "&.cm-focused .cm-selectionBackground": {
        backgroundColor: "var(--accent-color-transparent) !important", // Same color for focused state
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
        backgroundColor: "transparent !important" // Removes the additional highlighting
    },
    
    ".cm-snippetField": {
        backgroundColor: "var(--accent-color-transparent) !important", // VERY noticeable color
        color: "white !important",            // So that the text remains readable
        outline: "2px dashed red !important", // Noticeable border
        borderRadius: "3px",
    },

    ".cm-snippetField-active": {
        backgroundColor: "lime !important",    // ANOTHER VERY noticeable color
        color: "black !important",
        outline: "2px solid blue !important", // Another noticeable border
    }
});

// Syntax-Highlighting für das dunkle Theme
export const darkHighlightStyle = HighlightStyle.define([
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

export const darkHighlighting = syntaxHighlighting(darkHighlightStyle);

// Lua keywords and functions for Autocomplete
// Lua keywords and functions for Autocomplete
export const luaCompletions = [
  {
  label: "press",
  type: "function",
  apply: snippet("press(${key})"),
  info: "press(key) - Simulates pressing a key. Key can be a string (e.g., 'a', 'ctrl') or a keycode number."
},
{
  label: "release",
  type: "function",
  apply: snippet("release(${key})"),
  info: "release(key) - Simulates releasing a key. Key can be a string (e.g., 'a', 'ctrl') or a keycode number."
},
{
  label: "tap",
  type: "function",
  apply: snippet("tap(${key}, ${delay})"),
  info: "tap(key, [delay]) - Simulates pressing and releasing a key. Key can be a string or a keycode, optional delay in ms."
},
{
  label: "wait",
  type: "function",
  apply: snippet("wait(${duration})"),
  info: "wait(duration) - Waits for the specified duration in milliseconds."
},
{
  label: "combo",
  type: "function",
  apply: snippet("combo({${key1}, ${key2}}, ${delay})"),
  info: "combo(keys, [delay]) - Simulates pressing multiple keys at once. Keys can be a table, string, or keycode, with optional delay."
},
{
  label: "flush",
  type: "function",
  apply: snippet("flush()"),
  info: "flush() - Releases all currently pressed keys."
},
{
  label: "exec_bash",
  type: "function",
  apply: snippet("exec_bash(${command}, {${\"output\"}})"),
  info: "exec_bash(command, [options]) - Executes a bash command. Options can include: 'output', 'wait', 'background', 'root'."
},
{
  label: "clipboard",
  type: "function",
  apply: snippet("clipboard()"),
  info: "clipboard() - Returns the current content of the system clipboard as a string."
},
  // Keywords with snippet templates
  {
    label: "if",
    type: "keyword",
    apply: snippet("if ${condition} then\n\t${}\nend"),
    detail: "If statement"
  },
  {
    label: "ifelse",
    type: "keyword",
    apply: snippet("if ${condition} then\n\t${}\nelse\n\t\nend"),
    detail: "If-Else statement"
  },
  {
    label: "ifelseif",
    type: "keyword",
    apply: snippet("if ${condition1} then\n\t${}\nelseif ${condition2} then\n\t\nelse\n\t\nend"),
    detail: "If-ElseIf-Else statement"
  },
  {
    label: "for",
    type: "keyword",
    apply: snippet("for ${i}=${1}, ${10} do\n\t${}\nend"),
    detail: "Numeric for loop"
  },
  {
    label: "fori",
    type: "keyword",
    apply: snippet("for ${index}, ${value} in ipairs(${myTable}) do\n\t${}\nend"),
    detail: "ipairs loop (indexed)"
  },
  {
    label: "forp",
    type: "keyword",
    apply: snippet("for ${key}, ${value} in pairs(${myTable}) do\n\t${}\nend"),
    detail: "pairs loop (all keys)"
  },
  {
    label: "while",
    type: "keyword",
    apply: snippet("while ${condition} do\n\t${}\nend"),
    detail: "While loop"
  },
  {
    label: "repeat",
    type: "keyword",
    apply: snippet("repeat\n\t${}\nuntil ${condition}"),
    detail: "Repeat-Until loop"
  },
  {
    label: "function",
    type: "keyword",
    apply: snippet("function ${myFunction}()\n\t${}\nend"),
    detail: "Function definition"
  },
  {
    label: "localfunc",
    type: "keyword",
    apply: snippet("local function ${myFunction}()\n\t${}\nend"),
    detail: "Local function definition"
  },
  {
    label: "method",
    type: "keyword",
    apply: snippet("function ${myTable}:${myMethod}()\n\t${}\nend"),
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
    apply: snippet("local ${myTable} = {\n\t${}\n}"),
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
  { label: "return", type: "keyword", apply: snippet("return ${value}")}, // Optional with field
  { label: "then", type: "keyword" },
  { label: "true", type: "keyword" },
  { label: "until", type: "keyword" },

  {
    label: "print_var",
    type: "snippet",
    apply: snippet("print(\"${variableName}:\", ${variableName})"),
    detail: "Print variable with label"
  },
  {
    label: "error_handle",
    type: "snippet",
    apply: snippet("local status, result = pcall(function()\n\t${}\nend)\nif not status then\n\tprint(\"Error:\", result)\nend"),
    detail: "Error handling with pcall"
  },
  {
    label: "module",
    type: "snippet",
    apply: snippet("local ${MyModule} = {}\n\nfunction ${MyModule}.${myFunction}(${param1, param2})\n\t${}\nend\n\nreturn ${MyModule}"),
    detail: "Module pattern"
  },
  {
    label: "class",
    type: "snippet",
    apply: snippet("local ${MyClass} = {}\n${MyClass}.__index = ${MyClass}\n\nfunction ${MyClass}.new(${param1, param2})\n\tlocal self = setmetatable({}, ${MyClass})\n\t${}\n\treturn self\nend\n\nfunction ${MyClass}:${myMethod}()\n\t${cursor2}\nend\n\nreturn ${MyClass}"),
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
  { label: "math.fmod", type: "function", apply: snippet("math.fmod(${x}, ${y})"), info: "math.fmod(x,y) - Remainder of x/y." }, // Hinzugefügt, da oft genutzt
  { label: "math.huge", type: "constant", info: "Value representing infinity." }, // Konstante, kein Snippet
  { label: "math.log", type: "function", apply: snippet("math.log(${x}, ${base})"), info: "math.log(x [, base]) - Logarithm." },
  { label: "math.max", type: "function", apply: snippet("math.max(${x}, ${...})"), info: "math.max(x, ...) - Maximum value." },
  { label: "math.min", type: "function", apply: snippet("math.min(${x}, ${...})"), info: "math.min(x, ...) - Minimum value." },
  { label: "math.modf", type: "function", apply: snippet("math.modf(${x})"), info: "math.modf(x) - Returns integral and fractional parts of x."}, // Added
  { label: "math.pi", type: "constant", info: "Mathematical constant pi." }, // Konstante, kein Snippet
  { label: "math.pow", type: "function", apply: snippet("math.pow(${x}, ${y})"), info: "math.pow(x,y) - x^y (same as x^y)."}, // Added (x^y is more common)
  { label: "math.rad", type: "function", apply: snippet("math.rad(${x})"), info: "math.rad(x) - Degrees to radians." },
  { label: "math.random", type: "function", apply: snippet("math.random(${m}, ${n})"), info: "math.random([m [, n]]) - Random number." },
  { label: "math.randomseed", type: "function", apply: snippet("math.randomseed(${x})"), info: "math.randomseed(x) - Sets random seed." },
  { label: "math.sin", type: "function", apply: snippet("math.sin(${x})"), info: "math.sin(x) - Sine." },
  { label: "math.sqrt", type: "function", apply: snippet("math.sqrt(${x})"), info: "math.sqrt(x) - Square root." },
  { label: "math.tan", type: "function", apply: snippet("math.tan(${x})"), info: "math.tan(x) - Tangent." },
  { label: "math.tointeger", type: "function", apply: snippet("math.tointeger(${x})"), info: "math.tointeger(x) - Converts x to an integer, or nil if not convertible."}, // Lua 5.3+
  { label: "math.type", type: "function", apply: snippet("math.type(${x})"), info: "math.type(x) - Returns 'integer', 'float', or nil if x is not a number."}, // Lua 5.3+
  { label: "math.ult", type: "function", apply: snippet("math.ult(${m}, ${n})"), info: "math.ult(m,n) - Returns boolean indicating if m < n as unsigned integers."}, // Lua 5.3+
];


// Configure Autocomplete Extension
export const myCompletions = autocompletion({
    override: [
        (context) => {
            const state = context.state;
            
            // Check if active snippet fields exist in the editor
            if (hasNextSnippetField(state) || hasPrevSnippetField(state)) {
                // We are in a snippet field, so do not offer Autocomplete
                return null;
            }
            
            // Normal Autocomplete when not in a snippet field
            let word = context.matchBefore(/\w*/);
            if (!word || (word.from == word.to && !context.explicit))
                return null;
            return {
                from: word.from,
                options: luaCompletions,
                span: /^\w*$/
            };
        }

    ]
});

let editorView: EditorView | null = null;

// Export extensions as an array for easy use
export const cmExtensions = [
    luaLanguage,
    darkTheme,
    darkHighlighting,
    myCompletions,
    EditorView.updateListener.of(update => {
      if (update.view) {
        editorView = update.view;
      }
    })
];


export function  performUndo() {
  if (editorView) {
    undo(editorView);
  }
}


  
export function performRedo(): void {
  if (editorView) {
    redo(editorView);
  }
}

export function performSelectAll(): void {
  if (editorView) {
    selectAll(editorView);
  }
}

export function performCopy(): boolean {
  if (editorView && editorView.hasFocus) {
    try {
      return document.execCommand('copy');
    } catch (e) {
      return false;
    }
  }
  return false;
}

export function performPaste(): boolean {
  if (editorView && editorView.hasFocus) {
    try {
      return document.execCommand('paste');
    } catch (e) {
      return false;
    }
  }
  return false;
}