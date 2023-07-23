- cd into dir
- fire off hook

- uvem happens

  - grab `cwd`
  - walk dir

  - for each path:
    - check if path is `$virtual_env`
  
  - for each `$virtual_env`
    - check if enabled
    - enable

  - for each enabled virtual env enabled
    - if not in list of current venvs
      - deactivate