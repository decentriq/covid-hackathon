use super::Cipher;
use super::Keypair;
use super::Nonce;

use pyo3::prelude::*;

use x25519_dalek::PublicKey;
use x25519_dalek::StaticSecret;

#[pymodule]
fn chily(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNonce>()?;
    m.add_class::<PyStaticSecret>()?;
    m.add_class::<PyPublicKey>()?;
    m.add_class::<PyKeypair>()?;
    m.add_class::<PyCipher>()?;
    Ok(())
}

#[pyclass(name=Nonce)]
pub struct PyNonce {
    inner: Nonce,
}

impl From<Nonce> for PyNonce {
    fn from(nonce: Nonce) -> Self {
        Self { inner: nonce }
    }
}

#[pymethods]
impl PyNonce {
    #[staticmethod]
    pub fn from_random() -> PyResult<PyNonce> {
        Ok(Nonce::from_random().into())
    }

    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>) -> PyResult<PyNonce> {
        let nonce: Nonce = bytes[..].into();
        Ok(nonce.into())
    }

    #[getter]
    pub fn bytes(&self) -> PyResult<Vec<u8>> {
        let bytes = self.inner.bytes;
        Ok(bytes.to_vec())
    }
}

#[pyclass(name=StaticSecret)]
pub struct PyStaticSecret {
    inner: StaticSecret,
}

impl From<StaticSecret> for PyStaticSecret {
    fn from(secret: StaticSecret) -> Self {
        Self { inner: secret }
    }
}

#[pymethods]
impl PyStaticSecret {
    #[getter]
    pub fn bytes(&self) -> PyResult<Vec<u8>> {
        let bytes = self.inner.to_bytes();
        Ok(bytes.to_vec())
    }

    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>) -> PyResult<PyStaticSecret> {
        let mut buffer: [u8; 32] = [0; 32];
        buffer.copy_from_slice(&bytes[0..32]);
        let secret: StaticSecret = buffer.into();
        Ok(secret.into())
    }
}

#[pyclass(name=PublicKey)]
pub struct PyPublicKey {
    inner: PublicKey,
}

impl From<PublicKey> for PyPublicKey {
    fn from(public_key: PublicKey) -> Self {
        Self { inner: public_key }
    }
}

#[pymethods]
impl PyPublicKey {
    #[getter]
    pub fn bytes(&self) -> PyResult<Vec<u8>> {
        let bytes = self.inner.as_bytes();
        Ok(bytes.to_vec())
    }

    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>) -> PyResult<PyPublicKey> {
        let mut buffer: [u8; 32] = [0; 32];
        buffer.copy_from_slice(&bytes[0..32]);
        let public_key: PublicKey = buffer.into();
        Ok(public_key.into())
    }
}

#[pyclass(name=Keypair)]
pub struct PyKeypair {
    inner: Keypair,
}

#[pymethods]
impl PyKeypair {
    #[staticmethod]
    pub fn from_random() -> PyResult<PyKeypair> {
        let key_pair = Keypair::generate();
        Ok(PyKeypair { inner: key_pair })
    }

    #[staticmethod]
    pub fn from_secret(py_secret: &PyStaticSecret) -> PyResult<PyKeypair> {
        let public = PublicKey::from(&py_secret.inner);
        // no `Copy` implemented for `StaticSecret`
        let bytes = py_secret.inner.to_bytes();
        let secret: StaticSecret = bytes.into();
        Ok(PyKeypair {
            inner: Keypair { secret, public },
        })
    }

    #[getter]
    pub fn secret(&self) -> PyResult<PyStaticSecret> {
        // no `Copy` implemented for `StaticSecret`
        let bytes = self.inner.secret.to_bytes();
        let secret: StaticSecret = bytes.into();
        Ok(secret.into())
    }

    #[getter]
    pub fn public_key(&self) -> PyResult<PyPublicKey> {
        Ok(self.inner.public.into())
    }
}

#[pyclass(name = Cipher)]
pub struct PyCipher {
    inner: Cipher,
}

#[pymethods]
impl PyCipher {
    #[new]
    pub fn new(obj: &PyRawObject, my_secret_key: &PyStaticSecret, their_pub_key: &PyPublicKey) {
        obj.init({
            PyCipher {
                inner: Cipher::new(&my_secret_key.inner, &their_pub_key.inner),
            }
        })
    }

    pub fn encrypt(&mut self, data: Vec<u8>, nonce: &PyNonce) -> PyResult<Vec<u8>> {
        Ok(self.inner.encrypt(&data, &nonce.inner))
    }

    pub fn decrypt(&mut self, data: Vec<u8>, nonce: &PyNonce) -> PyResult<Vec<u8>> {
        Ok(self.inner.decrypt(&data, &nonce.inner))
    }
}
