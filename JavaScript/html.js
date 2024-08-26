class HtmlHelpers {
    static renderSelect(id, callback, options, selected) {
        let renderer = `<select id="${id}" class="form-select"  onchange="${callback}">`;
        for (let item of options) {
            if (selected == item) {
                renderer += '<option value="' + item + '" selected>' + item + "</option>";
            }
            else {
                renderer += '<option value="' + item + '">' + item + "</option>";
            }
        }
        return renderer + "</select>";
    }
    static renderProducts(products) {
        let selectedProduct = AppContext.getSelectedProduct();
        let result = `<select class="form-select" style="width:100%" onchange="AppContext.onProductSelect(this)">`;
        for (let product of products) {
            if (product == selectedProduct) {
                result += `<option selected>${product}</option>`;
            }
            else {
                result += `<option>${product}</option>`;
            }
        }
        return result + `</select>`;
    }
}
//# sourceMappingURL=html.js.map