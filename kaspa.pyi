from enum import Enum
from typing import Any, Callable, Iterator, Optional, TypedDict, Union

class Resolver:

    def __init__(self, urls: Optional[list[str]] = None, tls: Optional[int] = None) -> None: ...

    def urls(self) -> list[str]: ...

    def get_node(self, encoding: str, network_id: str) -> dict: ...

    def get_url(self, encoding: str, network_id: str) -> str: ...

    def connect(self, encoding: str, network_id: str) -> RpcClient: ...


class RpcClient:

    def __init__(self, resolver: Optional[Resolver] = None, url: Optional[str] = None, encoding: Optional[str] = None, network_id: Optional[str] = None) -> None: ...

    @property
    def url(self) -> str: ...

    @property
    def resolver(self) -> Optional[Resolver]: ...

    def set_resolver(self, Resolver) -> None: ...

    def set_network_id(self, network_id: str) -> None: ...

    @property
    def is_connected(self) -> bool: ...

    @property
    def encoding(self) -> str: ...

    @property
    def node_id(self) -> str: ...

    async def connect(self, block_async_connect: Optional[bool] = None, strategy: Optional[str] = None, url: Optional[str] = None, timeout_duration: Optional[int] = None, retry_interval: Optional[int] = None) -> None: ...

    async def disconnect(self) -> None: ...

    async def start(self) -> None: ...

    # def trigger_abort(self) -> None: ...

    def add_event_listener(self, event: str, callback: Callable[..., Any], *args: Any, **kwargs: Optional[Any]) -> None: ...

    def remove_event_listener(self, event: str, callback: Callable[..., Any] = None) -> None: ...

    def remove_all_event_listeners(self) -> None: ...

    # @staticmethod
    # def default_port(encoding: str, network: str) -> int: ...

    # @staticmethod
    # def parse_url(url: str, encoding: str, network: str) -> str: ...

    async def subscribe_utxos_changed(self, addresses: list[Address]) -> None: ...
    
    async def unsubscribe_utxos_changed(self, addresses: list[Address]) -> None: ...

    async def subscribe_virtual_chain_changed(self, include_accepted_transaction_ids: bool) -> None: ...
    
    async def unsubscribe_virtual_chain_changed(self, include_accepted_transaction_ids: bool) -> None: ...

    async def subscribe_block_added(self) -> None: ...
    
    async def unsubscribe_block_added(self) -> None: ...

    async def subscribe_finality_conflict(self) -> None: ...
    
    async def unsubscribe_finality_conflict(self) -> None: ...

    async def subscribe_finality_conflict_resolved(self) -> None: ...
    
    async def unsubscribe_finality_conflict_resolved(self) -> None: ...

    async def subscribe_new_block_template(self) -> None: ...
    
    async def unsubscribe_new_block_template(self) -> None: ...

    async def subscribe_pruning_point_utxo_set_override(self) -> None: ...
    
    async def unsubscribe_pruning_point_utxo_set_override(self) -> None: ...
    
    async def subscribe_sink_blue_score_changed(self) -> None: ...
    
    async def unsubscribe_sink_blue_score_changed(self) -> None: ...
    
    async def subscribe_virtual_daa_score_changed(self) -> None: ...
    
    async def unsubscribe_virtual_daa_score_changed(self) -> None: ...

    async def get_block_count(self) -> dict: ...
    
    async def get_block_dag_info(self) -> dict: ...
    
    async def get_coin_supply(self) -> dict: ...
    
    async def get_connected_peer_info(self) -> dict: ...
    
    async def get_info(self) -> dict: ...
    
    async def get_peer_addresses(self) -> dict: ...
            
    async def get_sink(self) -> dict: ...
    
    async def get_sink_blue_score(self) -> dict: ...
    
    async def ping(self) -> dict: ...
    
    async def shutdown(self) -> dict: ...
    
    async def get_server_info(self) -> dict: ...
    
    async def get_sync_status(self) -> dict: ...
    
    async def get_fee_estimate(self) -> dict: ...

    async def get_current_network(self) -> dict: ...

    async def get_system_info(self) -> dict: ...

    async def add_peer(self, request: dict) -> dict: 
        """
        Args:
            request (dict): Containing keys:
                - peer_address (str): IP address and port of the peer (e.g., "192.168.1.1:16111")
                - is_permanent (bool): Whether this peer should be treated as permanent
        """
        ...
    
    async def ban(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - ip (str): IP address to ban (e.g., "192.168.1.1")
        """
        ...
    
    async def estimate_network_hashes_per_second(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - windowSize (int): Number of blocks to use for estimation
                - startHash (str, optional): Block hash to start estimation from
        """
        ...
    
    async def get_balance_by_address(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - address (str): Kaspa address to get balance for
        """
        ...
    
    async def get_balances_by_addresses(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - addresses (list[str]): List of Kaspa addresses to get balances for
        """
        ...
    
    async def get_block(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - hash (str): Block hash to retrieve
                - includeTransactions (bool): Whether to include transaction data
        """
        ...
    
    async def get_blocks(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - lowHash (str, optional): Starting block hash
                - includeBlocks (bool): Whether to include block data
                - includeTransactions (bool): Whether to include transaction data
        """
        ...
    
    async def get_block_template(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - payAddress (str): Address to receive coinbase rewards
                - extraData (list[int]): Additional data to include in coinbase (as bytes)
        """
        ...
    
    async def get_connections(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - includeProfileData (bool): Whether to include profiling information
        """
        ...

    async def get_current_block_color(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - hash (str): Block hash to get color for
        """
        ...
    
    async def get_daa_score_timestamp_estimate(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - daaScores (list[int]): List of DAA scores to get timestamp estimates for
        """
        ...
    
    async def get_fee_estimate_experimental(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - verbose (bool): Whether to include verbose fee estimation data
        """
        ...

    async def get_headers(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - startHash (str): Starting block hash
                - limit (int): Maximum number of headers to return
                - isAscending (bool): Whether to return headers in ascending order
        """
        ...
    
    async def get_mempool_entries(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - includeOrphanPool (bool): Whether to include orphan transactions
                - filterTransactionPool (bool): Whether to filter transaction pool
        """
        ...
    
    async def get_mempool_entries_by_addresses(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - addresses (list[str]): List of addresses to filter mempool entries
                - includeOrphanPool (bool): Whether to include orphan transactions
                - filterTransactionPool (bool): Whether to filter transaction pool
        """
        ...
    
    async def get_mempool_entry(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - transactionId (str): Transaction ID to look up in mempool
                - includeOrphanPool (bool): Whether to include orphan pool in search
                - filterTransactionPool (bool): Whether to filter transaction pool
        """
        ...
    
    async def get_metrics(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - processMetrics (bool): Whether to include process metrics
                - connectionMetrics (bool): Whether to include connection metrics
                - bandwidthMetrics (bool): Whether to include bandwidth metrics
                - consensusMetrics (bool): Whether to include consensus metrics
                - storageMetrics (bool): Whether to include storage metrics
                - customMetrics (bool): Whether to include custom metrics
        """
        ...

    async def get_subnetwork(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - subnetworkId (str): Subnetwork ID to retrieve information for
        """
        ...
    
    async def get_utxos_by_addresses(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - addresses (list[str]): List of addresses to get UTXOs for
        """
        ...

    async def get_utxo_return_address(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - txid (str): Transaction ID
                - acceptingBlockDaaScore (int): Accepting block DAA score
        """
        ...

    async def get_virtual_chain_from_block(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - startHash (str): Starting block hash
                - includeAcceptedTransactionIds (bool): Whether to include accepted transaction IDs
                - minConfirmationCount (int, optional): Minimum confirmation count
        """
        ...
    
    async def resolve_finality_conflict(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - finalityBlockHash (str): Hash of the finality block to resolve conflict with
        """
        ...
    
    async def submit_block(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - block (dict): Complete block data including header and transactions
                - allowNonDaaBlocks (bool): Whether to allow non-DAA blocks
        """
        ...
    
    async def submit_transaction(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - transaction (dict): Complete transaction data
                - allowOrphan (bool): Whether to allow orphan transactions
        """
        ...
    
    async def submit_transaction_replacement(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - transaction (dict): Complete replacement transaction data
        """
        ...

    async def unban(self, request: dict) -> dict:
        """
        Args:
            request (dict): Containing keys:
                - ip (str): IP address to unban (e.g., "192.168.1.1")
        """
        ...
