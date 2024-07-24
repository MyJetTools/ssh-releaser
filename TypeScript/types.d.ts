declare const $;


interface ReleaseAppsHttpModel {
    ids: IdGroupHttpModel[];
    labels: string[];
}


interface IdGroupHttpModel {
    category: string;
    ids: ReleaseStepHttpModel[];
}


interface ReleaseStepHttpModel {
    id: string;
    exclude_features: string[],
    include_features: string[],
}

interface IEnvironment {
    id: String,
    features: String[];
}