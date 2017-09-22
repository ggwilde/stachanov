# Design Proposal 0001 - Transaction relationships

Like in any database system different transactions in the blockchain must be able to relate to each other. In contrast to currency apps like Bitcoin, where mostly simple 1:1 relationships of transaction inputs and remittances are implemented, transactions in stachanov are more complex and can relate in various ways to each other. As a speaking convention in stachanov only *transaction relationships* can be claimed, not transactions themselves.

A transaction developer can add different relationships to each transaction type, similar to ForeignKeys in relational databases. These relationships can then be claimed by other transactions in later blocks. Relationships have a distinct relationship id and are defined on the target of a reference, meaning that each transaction type must disclose the relationships that others can claim.

## Total relationship state

Transactions have a total relationship state, which is prioritized over the state of single relationships. This is provided to account for transaction finalization, when a special transaction disables all further claims to relationships of another transaction. Think about a transaction that registers a production start of some sort. The production collective can add workload allocations that refer to the production start transaction as long as the production has not been finished, that is: finalized by a production output transaction.

## 1:1 relationships

1:1 relationships can be claimed *exactly* once. A typical example would be the transformation from workloads into coupons (every workload can only be claimed by one coupon) or the parent relationship of a collective (every collective can have exactly one parent, making sure, that collectives only form well-defined trees)

## 1:n relationships

1:n relationships can be claimed by multiple transactions, e.g. the workloads relationship of a production start transaction can be claimed by multiple workload transactions. 1:n relationships have no inherent limit of referer transactions, meaning that they can be claimed until the transaction is finalized or a limit is imposed by the referer transaction logic.
