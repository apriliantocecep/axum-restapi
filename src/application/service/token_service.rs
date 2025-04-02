use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisResult};
use tokio::sync::MutexGuard;
use crate::application::constant::{JWT_REDIS_REVOKED_TOKENS_KEY, JWT_REDIS_REVOKE_GLOBAL_BEFORE, JWT_REDIS_REVOKE_USER_BEFORE_KEY};
use crate::application::security::jwt::ClaimsMethods;
use crate::application::state::SharedState;

pub async fn is_revoked<T: std::fmt::Debug + ClaimsMethods + Send + Sync> (
    claims: &T,
    state: &SharedState,
) -> RedisResult<bool> {
    let mut redis = state.cache.lock().await;

    let global_revoke = is_global_revoked(claims, &mut redis).await?;
    if global_revoke {
        tracing::error!("access denied (globally revoked): {:#?}", claims);
        return Ok(true)
    }

    let user_revoked = is_user_revoked(claims, &mut redis).await?;
    if user_revoked {
        tracing::error!("access denied (user revoked): {:#?}", claims);
        return Ok(true)
    }

    let user_revoked = is_user_revoked(claims, &mut redis).await?;
    if user_revoked {
        tracing::error!("access denied (user revoked): {:#?}", claims);
        return Ok(true)
    }

    let token_revoked = is_token_revoked(claims, &mut redis).await?;
    if token_revoked {
        tracing::error!("access denied (toke revoked): {:#?}", claims);
        return Ok(true)
    }

    drop(redis);
    Ok(false)
}

async fn is_token_revoked<T: ClaimsMethods + Send + Sync>(
    claims: &T,
    redis: &mut MutexGuard<'_, MultiplexedConnection>
) -> RedisResult<bool> {
    redis.hexists(JWT_REDIS_REVOKED_TOKENS_KEY, claims.get_jti()).await
}

async fn is_user_revoked<T: ClaimsMethods + Send + Sync>(
    claims: &T,
    redis: &mut MutexGuard<'_, MultiplexedConnection>
) -> RedisResult<bool> {
    let user_id = claims.get_sub();
    let opt_ext: Option<String> = redis.hget(JWT_REDIS_REVOKE_USER_BEFORE_KEY, user_id).await?;
    if let Some(exp) = opt_ext {
        let global_exp = exp.parse::<usize>().unwrap();
        if global_exp >= claims.get_iat() {
            return Ok(true)
        }
    }
    Ok(false)
}

async fn is_global_revoked<T: ClaimsMethods + Sync + Send>(
    claims: &T,
    redis: &mut MutexGuard<'_, MultiplexedConnection>,
) -> RedisResult<bool> {
    let opt_exp: Option<String> = redis.get(JWT_REDIS_REVOKE_GLOBAL_BEFORE).await?;
    if let Some(exp) = opt_exp {
        let global_exp = exp.parse::<usize>().unwrap();
        if global_exp >= claims.get_iat() {
            return Ok(true)
        }
    }

    Ok(false)
}