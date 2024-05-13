class HtmlUtils {



    static renderSplitTable(leftWidth: string, rightWidth: string, leftPanel: () => string, rightPanel: () => string): string {

        let result = `<table style="width:100%"><tr><td style="width:${leftWidth}">`;

        result += leftPanel();

        result += `</td><td style="width:${rightWidth}">`;

        result += rightPanel();

        return result + `</td></tr></table>`;

    }

    static render3Table(leftWidth: string, middleWidth: string, rightWidth: string, leftPanel: () => string, middlePanel: () => string, rightPanel: () => string): string {

        let result = `<table style="width:100%"><tr><td style="width:${leftWidth}">`;

        result += leftPanel();

        result += `</td><td style="width:${middleWidth}">`;

        result += middlePanel();

        result += `</td><td style="width:${rightWidth}">`;

        result += rightPanel();

        return result + `</td></tr></table>`;

    }

}