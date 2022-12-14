// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded a limit. Try making the call later. For information about limits, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>.</p>
    ClientLimitExceededException(crate::error::ClientLimitExceededException),
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p>The codec private data in at least one of the tracks of the video stream is not valid for this operation.</p>
    InvalidCodecPrivateDataException(crate::error::InvalidCodecPrivateDataException),
    /// <p>One or more frames in the requested clip could not be parsed based on the specified codec.</p>
    InvalidMediaFrameException(crate::error::InvalidMediaFrameException),
    /// <p>No codec private data was found in at least one of tracks of the video stream.</p>
    MissingCodecPrivateDataException(crate::error::MissingCodecPrivateDataException),
    /// <p>A streaming session was requested for a stream that does not retain data (that is, has a <code>DataRetentionInHours</code> of 0). </p>
    NoDataRetentionException(crate::error::NoDataRetentionException),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// <p> <code>GetMedia</code> throws this error when Kinesis Video Streams can't find the stream that you specified.</p>
    /// <p> <code>GetHLSStreamingSessionURL</code> and <code>GetDASHStreamingSessionURL</code> throw this error if a session with a <code>PlaybackMode</code> of <code>ON_DEMAND</code> or <code>LIVE_REPLAY</code>is requested for a stream that has no fragments within the requested time range, or if a session with a <code>PlaybackMode</code> of <code>LIVE</code> is requested for a stream that has no fragments within the last 30 seconds.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The type of the media (for example, h.264 or h.265 video or ACC or G.711 audio) could not be determined from the codec IDs of the tracks in the first fragment for a playback session. The codec ID for track 1 should be <code>V_MPEG/ISO/AVC</code> and, optionally, the codec ID for track 2 should be <code>A_AAC</code>.</p>
    UnsupportedStreamMediaTypeException(crate::error::UnsupportedStreamMediaTypeException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ClientLimitExceededException(inner) => inner.fmt(f),
            Error::InvalidArgumentException(inner) => inner.fmt(f),
            Error::InvalidCodecPrivateDataException(inner) => inner.fmt(f),
            Error::InvalidMediaFrameException(inner) => inner.fmt(f),
            Error::MissingCodecPrivateDataException(inner) => inner.fmt(f),
            Error::NoDataRetentionException(inner) => inner.fmt(f),
            Error::NotAuthorizedException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::UnsupportedStreamMediaTypeException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetClipError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetClipError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetClipError> for Error {
    fn from(err: crate::error::GetClipError) -> Self {
        match err.kind {
            crate::error::GetClipErrorKind::ClientLimitExceededException(inner) => {
                Error::ClientLimitExceededException(inner)
            }
            crate::error::GetClipErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::GetClipErrorKind::InvalidCodecPrivateDataException(inner) => {
                Error::InvalidCodecPrivateDataException(inner)
            }
            crate::error::GetClipErrorKind::InvalidMediaFrameException(inner) => {
                Error::InvalidMediaFrameException(inner)
            }
            crate::error::GetClipErrorKind::MissingCodecPrivateDataException(inner) => {
                Error::MissingCodecPrivateDataException(inner)
            }
            crate::error::GetClipErrorKind::NoDataRetentionException(inner) => {
                Error::NoDataRetentionException(inner)
            }
            crate::error::GetClipErrorKind::NotAuthorizedException(inner) => {
                Error::NotAuthorizedException(inner)
            }
            crate::error::GetClipErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetClipErrorKind::UnsupportedStreamMediaTypeException(inner) => {
                Error::UnsupportedStreamMediaTypeException(inner)
            }
            crate::error::GetClipErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDASHStreamingSessionURLError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetDASHStreamingSessionURLError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetDASHStreamingSessionURLError> for Error {
    fn from(err: crate::error::GetDASHStreamingSessionURLError) -> Self {
        match err.kind {
            crate::error::GetDASHStreamingSessionURLErrorKind::ClientLimitExceededException(inner) => Error::ClientLimitExceededException(inner),
            crate::error::GetDASHStreamingSessionURLErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
            crate::error::GetDASHStreamingSessionURLErrorKind::InvalidCodecPrivateDataException(inner) => Error::InvalidCodecPrivateDataException(inner),
            crate::error::GetDASHStreamingSessionURLErrorKind::MissingCodecPrivateDataException(inner) => Error::MissingCodecPrivateDataException(inner),
            crate::error::GetDASHStreamingSessionURLErrorKind::NoDataRetentionException(inner) => Error::NoDataRetentionException(inner),
            crate::error::GetDASHStreamingSessionURLErrorKind::NotAuthorizedException(inner) => Error::NotAuthorizedException(inner),
            crate::error::GetDASHStreamingSessionURLErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetDASHStreamingSessionURLErrorKind::UnsupportedStreamMediaTypeException(inner) => Error::UnsupportedStreamMediaTypeException(inner),
            crate::error::GetDASHStreamingSessionURLErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetHLSStreamingSessionURLError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetHLSStreamingSessionURLError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetHLSStreamingSessionURLError> for Error {
    fn from(err: crate::error::GetHLSStreamingSessionURLError) -> Self {
        match err.kind {
            crate::error::GetHLSStreamingSessionURLErrorKind::ClientLimitExceededException(inner) => Error::ClientLimitExceededException(inner),
            crate::error::GetHLSStreamingSessionURLErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
            crate::error::GetHLSStreamingSessionURLErrorKind::InvalidCodecPrivateDataException(inner) => Error::InvalidCodecPrivateDataException(inner),
            crate::error::GetHLSStreamingSessionURLErrorKind::MissingCodecPrivateDataException(inner) => Error::MissingCodecPrivateDataException(inner),
            crate::error::GetHLSStreamingSessionURLErrorKind::NoDataRetentionException(inner) => Error::NoDataRetentionException(inner),
            crate::error::GetHLSStreamingSessionURLErrorKind::NotAuthorizedException(inner) => Error::NotAuthorizedException(inner),
            crate::error::GetHLSStreamingSessionURLErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetHLSStreamingSessionURLErrorKind::UnsupportedStreamMediaTypeException(inner) => Error::UnsupportedStreamMediaTypeException(inner),
            crate::error::GetHLSStreamingSessionURLErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetImagesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetImagesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetImagesError> for Error {
    fn from(err: crate::error::GetImagesError) -> Self {
        match err.kind {
            crate::error::GetImagesErrorKind::ClientLimitExceededException(inner) => {
                Error::ClientLimitExceededException(inner)
            }
            crate::error::GetImagesErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::GetImagesErrorKind::NotAuthorizedException(inner) => {
                Error::NotAuthorizedException(inner)
            }
            crate::error::GetImagesErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetImagesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetMediaForFragmentListError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetMediaForFragmentListError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetMediaForFragmentListError> for Error {
    fn from(err: crate::error::GetMediaForFragmentListError) -> Self {
        match err.kind {
            crate::error::GetMediaForFragmentListErrorKind::ClientLimitExceededException(inner) => {
                Error::ClientLimitExceededException(inner)
            }
            crate::error::GetMediaForFragmentListErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::GetMediaForFragmentListErrorKind::NotAuthorizedException(inner) => {
                Error::NotAuthorizedException(inner)
            }
            crate::error::GetMediaForFragmentListErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetMediaForFragmentListErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListFragmentsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListFragmentsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListFragmentsError> for Error {
    fn from(err: crate::error::ListFragmentsError) -> Self {
        match err.kind {
            crate::error::ListFragmentsErrorKind::ClientLimitExceededException(inner) => {
                Error::ClientLimitExceededException(inner)
            }
            crate::error::ListFragmentsErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::ListFragmentsErrorKind::NotAuthorizedException(inner) => {
                Error::NotAuthorizedException(inner)
            }
            crate::error::ListFragmentsErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListFragmentsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
