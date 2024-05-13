class AppContext {
}
setTimeout(function () {
    $.ajax({ url: "/api/env/list" }).then(function (data) {
        AppContext.envs = data;
        Envs.refresh();
        Apps.init();
    });
}, 100);
//# sourceMappingURL=app.js.map