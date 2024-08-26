
class AppContext {
    static envs: {}; //todo!("Check how we are reading from here")
    static apps: SelectAdvanced<ReleaseStepHttpModel>;
    static labels: string[];
    static selectedProcess: string;

    static hideBackground() {
        let background = document.getElementById("background")
        background.classList.remove("visible");
        background.classList.add("hidden");

    }

    static getProducts(): string[] {
        return Object.keys(this.envs);
    }


    static getEnvs(product: string): IEnvironment[] {
        return this.envs[product];
    }


    static getSelectedProduct(): string {

        let result = localStorage.getItem("selectedProduct");

        if (!result) {
            result = this.getProducts()[0];
            localStorage.setItem("selectedProduct", result);
        }

        return result;
    }


    static onProductSelect(itm: HTMLInputElement) {
        let value = itm.value;
        console.log(value);
        localStorage.setItem("selectedProduct", value);
        Envs.refresh();
    }
}


setTimeout(function () {
    $.ajax({ url: "/api/env/list" }).then(function (data) {
        AppContext.envs = data;
        let products = AppContext.getProducts();
        let productsSelect = HtmlHelpers.renderProducts(products);
        document.getElementById("product-select-panel").innerHTML = productsSelect;
        Envs.refresh();
        Apps.init();
    });
}, 100);


setInterval(function () {

    if (AppContext.selectedProcess) {
        $.ajax({ url: "/api/release/logs", data: { id: AppContext.selectedProcess } }).then(function (data) {
            let doc = document.getElementById("content");
            doc.innerHTML = data.html;
            doc.scrollTo(0, doc.scrollHeight);

            if (data.finished == true) {
                AppContext.selectedProcess = undefined;
            }
        });
    }
}, 1000);



function backgroundClick() {
    let background = document.getElementById("background")
    background.classList.remove("visible");
    background.classList.add("hidden");
}
