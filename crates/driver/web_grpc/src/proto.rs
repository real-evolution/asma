use tonic::Status;

pub(super) mod value_types {
    tonic::include_proto!("driver_web_grpc.proto.value_types");
}

pub(super) mod models {
    tonic::include_proto!("driver_web_grpc.proto.models");
}

pub(super) mod services {
    tonic::include_proto!("driver_web_grpc.proto.services");
}

pub(super) type ProtoResult<T> = Result<T, Status>;
