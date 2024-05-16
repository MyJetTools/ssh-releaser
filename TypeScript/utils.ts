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


    static render4Table(widths: string[], p1: () => string, p2: () => string, p3: () => string, p4: () => string): string {

        let result = `<table style="width:100%"><tr><td style="width:${widths[0]}">`;

        result += p1();

        result += `</td><td style="width:${widths[1]}">`;

        result += p2();

        result += `</td><td style="width:${widths[2]}">`;

        result += p3();

        result += `</td><td style="width:${widths[3]}">`;

        result += p4();

        return result + `</td></tr></table>`;

    }

}