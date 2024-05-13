class HtmlUtils {



    static renderSplitTable(leftWidth: string, rightWidth: string, leftPanel: () => string, rightPanel: () => string): string {

        let result = `<table style="width:100%"><tr><td style="width:${leftWidth}">`;

        result += leftPanel();

        result += `</td><td style="width:${rightWidth}">`;

        result += rightPanel();

        return result + `</td></tr></table>`;

    }

}