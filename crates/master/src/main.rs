mod master;

use tonic::{transport::Server, Request, Response, Status};

use mr_common::task::{
    TaskRunRequest, TaskRunReply, TaskDesc, TaskType,
    RegisterWorkerRequest, RegisterWorkerReply,
    TaskStateRequest, TaskStateReply,
};
use mr_common::task::task_rpc_server::{TaskRpc, TaskRpcServer};


#[derive(Debug, Default)]
pub struct MasterTaskRpc {}

#[tonic::async_trait]
impl TaskRpc for MasterTaskRpc {
    async fn ask_task_to_run(
        &self,
        request: Request<TaskRunRequest>,
    ) -> Result<Response<TaskRunReply>, Status> {
        println!("Got a request: {:?}", request);
        let m = request.get_ref();
        println!("Message: {:?}", m);
        let reply = TaskRunReply {
            task: Some(TaskDesc {
                id: "task_id_999".into(),
                task_type: TaskType::Map as i32,
                input_files: Vec::new(),
                output_files: Vec::new(),
                reduce_num: 10
            })
        };
        Ok(Response::new(reply))
    }

    async fn register_worker(
        &self,
        request: Request<RegisterWorkerRequest>,
    ) -> Result<Response<RegisterWorkerReply>, Status> {
        let reply = RegisterWorkerReply {};
        Ok(Response::new(reply))
    }

    async fn task_state_change(
        &self,
        request: Request<TaskStateRequest>
    ) -> Result<Response<TaskStateReply>, Status> {
        let reply = TaskStateReply {};
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let rpc = MasterTaskRpc::default();
    Server::builder()
        .add_service(TaskRpcServer::new(rpc))
        .serve(addr)
        .await?;
    Ok(())
}
