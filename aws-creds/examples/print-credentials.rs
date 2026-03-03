use awscreds::Credentials;

#[derive(Debug)]
enum Source {
    DirectEnv,
    Profile(&'static str),
    #[cfg(feature = "http-credentials")]
    StsEnv,
    #[cfg(feature = "http-credentials")]
    ContainerCredentialsProvider,
    #[cfg(feature = "http-credentials")]
    InstanceMetadataV2,
    #[cfg(feature = "http-credentials")]
    InstanceMetadataV1,
}

fn load_with_source() -> Result<(Credentials, Source), Box<dyn std::error::Error>> {
    // Follow the same order as `Credentials::new(None, None, None, None, None)`

    // 1. Explicit environment variables
    if let Ok(creds) = Credentials::from_env() {
        return Ok((creds, Source::DirectEnv));
    }

    // 2. Shared credentials file / profile (`~/.aws/credentials` or AWS_SHARED_CREDENTIALS_FILE)
    if let Ok(creds) = Credentials::from_profile(None) {
        return Ok((creds, Source::Profile("default (or AWS_PROFILE)")));
    }

    // 3. HTTP-based providers (enabled when `http-credentials` is on)
    #[cfg(feature = "http-credentials")]
    {
        if let Ok(creds) = Credentials::from_sts_env("print-credentials-example") {
            return Ok((creds, Source::StsEnv));
        }

        if let Ok(creds) = Credentials::from_container_credentials_provider() {
            return Ok((creds, Source::ContainerCredentialsProvider));
        }

        if let Ok(creds) = Credentials::from_instance_metadata_v2(false) {
            return Ok((creds, Source::InstanceMetadataV2));
        }

        if let Ok(creds) = Credentials::from_instance_metadata(false) {
            return Ok((creds, Source::InstanceMetadataV1));
        }
    }

    Err("No credentials found in any supported source".into())
}

fn describe_source(source: &Source) -> &'static str {
    match source {
        Source::DirectEnv => "environment variables (e.g. AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY)",
        Source::Profile(_) => "shared credentials file (~/.aws/credentials or AWS_SHARED_CREDENTIALS_FILE)",
        #[cfg(feature = "http-credentials")]
        Source::StsEnv => "STS web identity (AWS_ROLE_ARN / AWS_WEB_IDENTITY_TOKEN_FILE)",
        #[cfg(feature = "http-credentials")]
        Source::ContainerCredentialsProvider => {
            "container credentials provider (ECS task role / EKS Pod Identity)"
        }
        #[cfg(feature = "http-credentials")]
        Source::InstanceMetadataV2 => "EC2 instance metadata service v2",
        #[cfg(feature = "http-credentials")]
        Source::InstanceMetadataV1 => "EC2 instance metadata service v1",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (creds, source) = load_with_source()?;

    println!("Loaded AWS credentials:");
    println!("  Access key:      {:?}", creds.access_key);
    println!("  Secret key:      {:?}", creds.secret_key);
    println!("  Security token:  {:?}", creds.security_token);
    println!("  Session token:   {:?}", creds.session_token);
    println!("  Expiration:      {:?}", creds.expiration);
    println!();
    println!("Credentials were retrieved from: {}", describe_source(&source));

    Ok(())
}

