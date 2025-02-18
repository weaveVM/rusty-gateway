use anyhow::Error;
use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::tx_envelope_writer::TxEnvelopeWrapper;

pub async fn retrieve_bundle_envelopes(txid: String) -> Result<Vec<TxEnvelopeWrapper>, Error> {
    let bundle = Bundle::retrieve_envelopes(txid).await?;
    Ok(bundle.envelopes)
}
