use diesel::{associations::HasTable, BoolExpressionMethods, ExpressionMethods};
use router_env::tracing::{self, instrument};

use super::generics;
use crate::{
    errors,
    payment_intent::{
        PaymentIntent, PaymentIntentNew, PaymentIntentUpdate, PaymentIntentUpdateInternal,
    },
    schema::payment_intent::dsl,
    CustomResult, PgPooledConn,
};

impl PaymentIntentNew {
    #[instrument(skip(conn))]
    pub async fn insert(
        self,
        conn: &PgPooledConn,
    ) -> CustomResult<PaymentIntent, errors::DatabaseError> {
        generics::generic_insert(conn, self).await
    }
}

impl PaymentIntent {
    #[instrument(skip(conn))]
    pub async fn update(
        self,
        conn: &PgPooledConn,
        payment_intent: PaymentIntentUpdate,
    ) -> CustomResult<Self, errors::DatabaseError> {
        match generics::generic_update_with_results::<<Self as HasTable>::Table, _, _, _>(
            conn,
            dsl::payment_id
                .eq(self.payment_id.to_owned())
                .and(dsl::merchant_id.eq(self.merchant_id.to_owned())),
            PaymentIntentUpdateInternal::from(payment_intent),
        )
        .await
        {
            Err(error) => match error.current_context() {
                errors::DatabaseError::NoFieldsToUpdate => Ok(self),
                _ => Err(error),
            },
            Ok(mut payment_intents) => payment_intents
                .pop()
                .ok_or(error_stack::report!(errors::DatabaseError::NotFound)),
        }
    }

    #[instrument(skip(conn))]
    pub async fn find_by_payment_id_merchant_id(
        conn: &PgPooledConn,
        payment_id: &str,
        merchant_id: &str,
    ) -> CustomResult<Self, errors::DatabaseError> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::payment_id.eq(payment_id.to_owned())),
        )
        .await
    }

    #[instrument(skip(conn))]
    pub async fn find_optional_by_payment_id_merchant_id(
        conn: &PgPooledConn,
        payment_id: &str,
        merchant_id: &str,
    ) -> CustomResult<Option<Self>, errors::DatabaseError> {
        generics::generic_find_one_optional::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::payment_id.eq(payment_id.to_owned())),
        )
        .await
    }
}
