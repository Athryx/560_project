# 560_project

In this project, we modify the AutoVerus pipeline by integrating Symbolic Execution tools like Seahorn and Crucible to generate correct specifications of Rust codes.

## Steps to Run the Pipeline

(All the commands here are given for a linux terminal)

## Initial buildup

### Option 1. Building docker image

```
git clone git@github.com:athryx/560_project
cd 560_project
git submodule update --init --recursive
cd ./submodules/verus-proof-synthesis
git checkout main
git pull
cd -
docker build -t autoverus-plus .
```

After the docker image is built, start a container,
```
docker run -it autoverus-plus /bin/bash
```

### Option 2. Using the existing docker image

Run the following commands

```
docker pull faaizmemon/560_project
docker run -it faaizmemon/560_project
```

## Executing the pipeline on the test cases 

Inside the docker node, navigate to code directory, and open the config file needed

```
cd code
vim config-artifact-openai.json
```

Add your OpenAI API key in the "aoai_api_key" field in the file 'config-artifact-openai.json'

In case you want to run the pipeline on a batch of test cases without annotation, run the following command:

```
./run_batch.sh ${DIRECTORY_PATH} ${OPTIONAL_KWARGS}
```

For example:
```
./run_batch.sh ../test_cases/LIA_single_loop 1 --with-smt2 --with-crux
```

If you have annotated test cases, you can add the ```--annotated``` flag.

The results will be generated in the code directory, ended in *_verified.rs or *_verified_crux.rs etc. format, depending on the keyword arguments. 
Then you can run Verus on the generated output, for example:

```
verus sum_0_to_n.rs
```

