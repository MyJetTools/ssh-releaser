# Initial settings

Application gets initial settings from a ~/.ssh-releaser file. File is a yaml file with the following structure:

```yaml
working_dir: ~/directory_for_your_scripts
home_dir: ~/home_directory_for_shared_files
global_vars: ~/global_vars.yaml
```

* working_dir - is a directory where all scripts are going to be stored. It's a mandatory field. The best practice would be to keep all the information which is going to be ok to publish to git repository;

* home_dir - the release scripts are going to be referencing to files which we do not want to store in public git repositories. To reference to this file - '~' at the start of the file path would reference to a home directory. For instance: ~/service_name/install.yaml. This file is going to be referenced to a file in a home directory which is specified in a home_dir field;

* global_vars - is a file with global variables. This file is going to be used for all scripts. It's a good place to store global variables which are not going to be published to cloud based git repositories not to share anything sensitive.

Format of global_vars file:

```yaml
vars:
  KEY: VALUE
  KEY2: VALUE2
```

# working_dir description.

working_dir - is a directory which is better to keep privately in git repositories. If git repository is going to be cloud based (not company in-house) such as github or gitlab - better not to keep anything sensitive in working_dir. Everything sensitive better goes to global_vars file or home_dir.

working_dir must contain a release.yaml yaml which is a starting point for release scripting.

Format of release.yaml file:

```yaml
vars:
  KEY: VALUE
  KEY2: VALUE2

var_files:
  - file1.yaml
  - file2.yaml

ssh:
  - id: VM-01
    host: 10.0.0.10
    port: 22
    user_name: user_name

  - id: VM-02
    host: 20.0.0.20
    port: 22
    user_name: root

  - id: VM-03
    host: 30.0.0.20
    port: 22
    user_name: root

execute_steps: ["*"]

steps:
  - id: install-service_1
    from_file: /service_1/install-script.yaml

  - id: install-service_2
    from_file: /service_2/install-script.yaml
```

Ssh connections are made with ssh_agent.


# Format of referenced var files

Section var_files gives ability to join variables from different files.

Example of external var file
```yaml
vars:
  KEY3: VALUE3
  KEY4: VALUE4 
```

# Format of Step file

Example: /service_name/install-script.yaml
```yaml
script:
  - type: execute
    ssh: VM-02
    commands:
      - name: Creating Folder for accounts-integration on VM-2
        exec: mkdir $HOME/services/service_name
        ignore_error: true
  - type: upload
    ssh: VM-02
    name: Uploading docker-compose file
    file:
      local_path: ./docker-compose.yaml
      remote_path: ~/services/service_name/docker-compose.yaml
      mode: 0o644
  - type: http_post
    name: Uploading settings
    ssh: VM-02
    post_data:
      url: ${SETTINGS_URL}/api/templates/post/
      headers:
        Content-Type: application/x-www-form-urlencoded
      body: env=env_name&name=service_name&yaml=${./settings.yaml:url_encoded:raw}

  - type: execute
    ssh: VM-02
    commands:
      - name: Pull Docker image
        exec: docker-compose -f $HOME/services/*{SERVICE_NAME}/docker-compose.yaml pull
        ignore_error: false

      - name: Kick off my-service-name
        exec: docker-compose -f $HOME/services/*{SERVICE_NAME}/docker-compose.yaml up -d
        ignore_error: false

  - type: from_template
    template_file_name: "/install-service-template.yaml"
    params:
      SSH_VM: VM-02
      SERVICE_NAME: my-service-name    

```

Please keep in mind:
* in case of upload type - local_path has format ./xxxxx which means file is taken to be uploaded is in the same directory as install-script.yaml step file;
* in case of http_post type - body has a ${./settings.yaml:url_encoded:raw} placeholder, which referees to a file in the same directory as install-script.yaml step file.


### example of template.yaml file

File: /install-service-template.yaml

```yaml
script:
  - type: execute
    ssh: *{SSH_VM}
    commands:
      - name: Creating Folder for *{SERVICE_NAME} on *{SSH_VM}
        exec: mkdir $HOME/services/*{SERVICE_NAME}
        ignore_error: true

  - type: upload
    ssh: *{SSH_VM}
    name: Uploading docker-compose file
    file:
      local_file: ./docker-compose.yaml
      remote_file: ~/services/*{SERVICE_NAME}/docker-compose.yaml
      mode: 0o644        

```

* when we are using from_template case and /install-service-template.yaml and /service_name/install-script.yaml are in different directories  ./xxxxx/file.yaml folder relates to the step file and going to be /service_name/xxxxx/file.yaml;
* params which are passed to a template file are going to be used as placeholders in a template file. In our example parameters are *{SSH_VM} and *{SERVICE_NAME}; When yaml is processed - Params  are populated first, placeholders second.




# Working with Placeholder.


Example of placeholder is: ${PLACEHOLDER_NAME}.

This means - it's going to be changed with a value of a variable with a key PLACEHOLDER_NAME. Variable can describe inside a release.yaml file or inside related file. 

If placeholder has to remain a placeholder - placeholder must be specified with the pattern{$PLACEHOLDER_NAME}. This placeholder is going to be changed to a  
${PLACEHOLDER_NAME} after processing

## Types of Placeholders

* ${PLACEHOLDER_NAME} - content is going to be taken either from variables of from env variables;
* ${/file_name} = content of placeholder is going to be taken from a file;
* ${./file_name} = content of placeholder is going to be taken from a file at the same folder as script;

* ${/file_name:url_encoded} = after reading - content is going to be url_encoded before it's going to be injected into a placeholder;
* ${/file_name:raw} = after reading from a file - if content has placeholders inside - they are not going to be populated by variables from files;

* ${/file_name:url_encoded:raw} = raw - makes sure that content from file is not altered from variables. url_encoded  - makes sure that content is url_encoded before it's going to be injected into a placeholder;


* *{PLACEHOLDER_NAME} - is a placeholder parameter which is used in case when yaml is used as a template and parameter has to be passed.

