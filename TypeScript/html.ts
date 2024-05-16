class HtmlHelpers {


    static renderSelect(id: string, callback: string, options: string[], selected: string): string {
        let renderer = `<select id="${id}" class="form-select"  onchange="${callback}">`;

        for (let item of options) {
            if (selected == item) {
                renderer += '<option value="' + item + '" selected>' + item + "</option>";
            } else {
                renderer += '<option value="' + item + '">' + item + "</option>";
            }

        }

        return renderer + "</select>";
    }
}