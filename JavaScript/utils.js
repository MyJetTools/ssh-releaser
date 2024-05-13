class HtmlUtils {
    static renderSplitTable(leftWidth, rightWidth, leftPanel, rightPanel) {
        let result = `<table style="width:100%"><tr><td style="width:${leftWidth}">`;
        result += leftPanel();
        result += `</td><td style="width:${rightWidth}">`;
        result += rightPanel();
        return result + `</td></tr></table>`;
    }
}
//# sourceMappingURL=utils.js.map