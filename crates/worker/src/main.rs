use mr_common::task::TaskRunRequest;
use mr_common::task::task_rpc_client::TaskRpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TaskRpcClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(TaskRunRequest{
        worker_id: "mr-req-111".into(),
    });
    let res = client.ask_task_to_run(request).await?;
    println!("Response from server: {:?}", res);
    Ok(())
}
