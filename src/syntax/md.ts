import {math} from 'mathlifier';
import {doInBetweenReplace} from "./syntax";

export function highlightMD(code: string, caretLine: number): string {
    let result = ""
    let lineno = 0

    for (let line of code.split("\n")) {

        // This needs to be transformed to a multiline statement
        // if (lineno == caretLine) {
        line = line.replace(/</g, "&lt;")
        line = line.replace(/>/g, "&gt;")
        // }

        if (line.startsWith("# ")) {
            let addClass = "target"

            if (lineno != caretLine) {
                line = line.replace("# ", "")
                addClass = ""
            }

            result += `<span class="syntax md h1 ${addClass}">${line}</span><br />`
        } else {
            let inlineCode = line.match(/`(.+?)`/g)
            line = line.replace(/`(.+?)`/g, "`<frag:inline>`")

            line = doInBetweenReplace(/\*\*([^*]+)\*\*/g, "bold", "**$1**", "md", lineno, caretLine, line)
            line = doInBetweenReplace(/\*([^*]+)\*(?!\*)/g, "italic", "*$1*", "md", lineno, caretLine, line)
            line = doInBetweenReplace(/~~([^~]+)~~/g, "strike", "~~$1~~", "md", lineno, caretLine, line)
            line = doInBetweenReplace(/__(.+?)__/g, "underline", "__$1__", "md", lineno, caretLine, line)

            // do latex math
            if (lineno != caretLine) {
                for (let match of line.matchAll(/\$([^$]+)\$/g)) {
                    let latex = match[1]
                    let result = math(latex)
                    line = line.replace(`$${latex}$`, `<span class="syntax md latex">${result}</span>`)
                }
            }

            if (inlineCode) {
                for (let i = 0; i < inlineCode.length; i++) {
                    if (lineno != caretLine) {
                        line = line.replace("`<frag:inline>`", `<span class="syntax md code">${inlineCode[i].replace(/`/g, "")}</span>`)
                    } else {
                        line = line.replace("`<frag:inline>`", `<span class="syntax md code target">${inlineCode[i]}</span>`)
                    }
                }
            }

            result += `${line}\n`
        }

        lineno++
    }

    return result
}