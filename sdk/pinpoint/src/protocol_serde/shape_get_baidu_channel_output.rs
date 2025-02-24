// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_baidu_channel_response_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::types::BaiduChannelResponse>,
    crate::operation::get_baidu_channel::GetBaiduChannelError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_baidu_channel_response::de_baidu_channel_response_payload(
                body,
            )
            .map_err(crate::operation::get_baidu_channel::GetBaiduChannelError::unhandled)
        })
        .transpose()
}
