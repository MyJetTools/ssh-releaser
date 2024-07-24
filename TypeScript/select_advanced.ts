
interface SelectItemsGroup<T> {
    name: string,
    items: T[]
}


interface SelectAdvancedSettings<T> {
    componentId: string,
    backgroundId: string,
    getItemAsHtml: (item: T) => string
    getItemValue: (item: T) => string;
    onSelect: (item: string) => void
}


class SelectAdvanced<T> {
    static enteredFilter: string;
    selected: string;
    static current: SelectAdvanced<any>;
    items: SelectItemsGroup<T>[];
    htmlComponent: Element;
    backgroundElement: Element;
    getItemAsHtml: (item: T) => string;
    getItemValue: (item: T) => string;
    onSelect: (item: string) => void;


    constructor(items: SelectItemsGroup<T>[], selected: string, setup: SelectAdvancedSettings<T>) {
        this.items = items;
        this.htmlComponent = document.getElementById(setup.componentId);
        this.backgroundElement = document.getElementById(setup.backgroundId);
        this.getItemAsHtml = setup.getItemAsHtml;
        this.onSelect = setup.onSelect;
        this.getItemValue = setup.getItemValue;
        SelectAdvanced.enteredFilter = "";
        SelectAdvanced.current = this;
        this.selected = selected;

        this.htmlComponent.addEventListener("click", () => {
            this.backgroundElement.classList.remove("hidden");
            this.backgroundElement.classList.add("visible");
            this.showContent();
        });
    }

    public dispose() {

    }

    public iterateItems(callback: (item: T) => void): void {
        for (let group of this.items) {
            for (let item of group.items) {
                callback(item);
            }
        }
    }

    renderThisItem(item: T) {
        let itemAsString = this.getItemValue(item).toLowerCase();
        //console.log("Item: " + itemAsString + ' selected: ' + SelectAdvanced.selected);
        return itemAsString.includes(SelectAdvanced.enteredFilter);
    }

    groupHasContentToShow(group: SelectItemsGroup<T>): boolean {

        if (SelectAdvanced.enteredFilter == "") {
            console.log("selected is empty");
            return true;
        }


        for (let item of group.items) {
            if (this.renderThisItem(item)) {
                return true;
            }
        }

        console.log("exiting at end");
        return false;

    }

    static onFilterChange(aThis) {
        SelectAdvanced.enteredFilter = aThis.value.toLowerCase();

        console.log("Filter changed: " + SelectAdvanced.enteredFilter);
        SelectAdvanced.current.reRenderItems();
    }

    public showContent(): void {

        let width = this.htmlComponent.clientWidth;

        let result = `<div class="select-popup" style="width:${width}px"  onclick="window.event.cancelBubble = true;"><div class="select-filter-panel" style="width:${width}px"><input class="select-filter" oninput="SelectAdvanced.onFilterChange(this)"/></div>`;

        result += `<div id="select-items">` + this.renderItems() + `</div>`;

        result += `</div>`;

        this.backgroundElement.innerHTML = result;

    }

    reRenderItems() {
        let items = document.getElementById("select-items");
        items.innerHTML = this.renderItems();
    }


    renderItems() {
        let result = "";
        for (let group of this.items) {

            if (!this.groupHasContentToShow(group)) {
                continue;
            }
            result += `<div class="select-group disable-selection">${group.name}</div>`;
            for (let item of group.items) {
                if (this.renderThisItem(item)) {

                    let selectedAttr = "";

                    if (this.selected == this.getItemValue(item)) {
                        selectedAttr = "select-item-selected";
                    }

                    let content = this.getItemAsHtml(item);
                    let value = this.getItemValue(item);

                    result += `<div data-value="${value}" class="select-item disable-selection ${selectedAttr}" onclick="SelectAdvanced.onClick(this)">${content}</div>`;

                }
            }
        }
        return result
    }

    static onClick(aThis) {
        let value = aThis.getAttribute("data-value");

        this.current.selectItem(value);

    }


    selectItem(value: string) {

        SelectAdvanced.enteredFilter = "";
        this.selected = value;
        this.htmlComponent.innerHTML = value;
        this.htmlComponent.setAttribute("data-value", value);
        AppContext.hideBackground();
        this.onSelect(value);
    }


}