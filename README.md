
# Working with Placeholder.


Example of placeholder is: ${PLACEHOLDER_NAME}.

This means - it's going to be changed with a value of a variable with a key PLACEHOLDER_NAME. Variable can describe inside a release.yaml file or inside related file. If we need to keep placeholder as a placeholder - placeholder must be specified as ${$PLACEHOLDER_NAME}. This placeholder is going to be changed to a  
${PLACEHOLDER_NAME} after processing

## Types of Placeholders

${PLACEHOLDER_NAME} - content is going to be taken either from variables of from env variables;
${/file_name} = content of placeholder is going to be taken from a file;
${/file_name:url_encoded} = after reading - content is going to be url_encoded before it's going to be injected into a placeholder;
