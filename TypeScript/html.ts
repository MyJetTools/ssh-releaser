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


    static renderProducts(products: string[]): string {

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