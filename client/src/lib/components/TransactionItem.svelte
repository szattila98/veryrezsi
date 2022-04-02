<script lang="ts">
	import { Transaction } from '$mock/api/models/expense_model';
	import IconButton from '@smui/icon-button';

	import { Item, Text, PrimaryText, SecondaryText } from '@smui/list';

	export let transaction: Transaction;
</script>

<Item style="width: 100%; display: flex; justify-content: space-between; align-items: center;">
	<Text>
		<PrimaryText>From {transaction.donorName} - {transaction.date}</PrimaryText>
		<SecondaryText>{transaction.value} {transaction.currencyType.abbreviation}</SecondaryText>
	</Text>
	<IconButton class="material-icons">close</IconButton>
</Item>

<style lang="scss">
</style>
