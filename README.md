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

### Running on a batch of test cases at a time
In case you want to run the pipeline on a batch of test cases, run the following command from the "code" directory:

```
./run_batch.sh ${DIRECTORY_PATH} ${OPTIONAL_KWARGS}
```

For example:
```
./run_batch.sh ../test_cases/LIA_single_loop 1 --with-smt2 --with-crux
```

If you have annotated test cases, you can add the ```--annotated``` flag.

### Running on one file
In case you want to test one file, run the following command from the "code" directory:

```
python main.py --input ${INPUT_DIRECTORY} --output ${OUTPUT_FILE_NAME} --config config-artifact-openai.json ${OPTIONAL_KWARGS}
```

For example:

```
python main.py --input ../test_cases/Nested_loops/nested_sum_1.rs --output nested_sum_1_verified_crux_smt2.rs --config config-artifact-openai.json --learning-type 1 --with-smt2 --with-crux
```

The results will be generated in the "code" directory, ended in *_verified.rs or *_verified_crux.rs etc. format, depending on the keyword arguments. 
Then you can run Verus on the generated output, for example:

```
verus nested_sum_1.rs
```

