use askama::Template;
use axum::{
    Form, Router,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use rand::seq::IndexedRandom;
use serde::Deserialize;
use time::Duration;

use crate::{
    app::AppState,
    auth::user::{UnauthenticatedUser, User},
    error::AppError,
    models::UserAsset,
    repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/dashboard", get(dashboard))
        .route("/logout", get(logout))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    repository: Repository,
    jar: CookieJar,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let unauth_user = UnauthenticatedUser::new(request.username, request.password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => unauth_user.register(&repository).await?,
        Err(other_err) => return Err(other_err),
    };

    let token = user.auth_token()?;

    let cookie = Cookie::build(("token", token)).http_only(true);

    Ok((jar.add(cookie), Redirect::to("/dashboard")))
}

async fn index(maybe_user: Option<User>) -> Result<Response, AppError> {
    match maybe_user {
        Some(_) => Ok(Redirect::to("/dashboard").into_response()),
        None => Ok(Redirect::to("/login").into_response()),
    }
}

async fn logout(jar: CookieJar) -> Result<impl IntoResponse, AppError> {
    let cookie = Cookie::build(("token", ""))
        .http_only(true)
        .max_age(Duration::seconds(0));
    Ok((jar.remove(cookie), Redirect::to("/login")))
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    user_name: String,
    assets: Vec<UserAsset>,
    total_carteira: String,
    curiosidade_historica: String,
}

const CURIOSIDADES_HISTORICAS: &[&str] = &[
    // Origens e Antiguidade
    "O Império Romano utilizava o 'Argentarii' como os primeiros banqueiros privados e negociadores de dívidas no fórum romano.",
    "As letras de câmbio foram inventadas por mercadores islâmicos durante a Idade Média (século VIII) para evitar o transporte físico de ouro em rotas comerciais.",
    "A primeira forma de seguro marítimo documentada remonta a cerca de 3.000 a.C. entre os mercadores fenícios e babilônios.",
    "O denário romano foi uma das primeiras moedas a sofrer desvalorização sistemática por inflação governamental sob o imperador Nero.",
    "A dinastia Song na China introduziu o 'Jiaozi' no século XI, sendo o primeiro papel-moeda oficial emitido por um governo.",
    "O termo 'banco' vem da palavra italiana banca, o balcão onde os cambistas medievais realizavam transações nas feiras livres.",
    "Os astecas utilizavam grãos de cacau como uma forma altamente valorizada de moeda e unidade de conta no México pré-colombiano.",
    // A Era das Bolsas e o Século XVII a XIX
    "A Bolsa de Amsterdã (1602) é considerada a primeira bolsa de valores moderna a negociar ações de forma contínua.",
    "O Acordo de Buttonwood, assinado em 1792 sob uma árvore em Nova York, deu origem à Bolsa de Valores de Nova York (NYSE).",
    "A Companhia do Mar do Sul protagonizou uma das maiores bolhas especulatórias da história britânica em 1720, levando Isaac Newton a declarar que conseguia calcular o movimento dos corpos celestes, mas não a loucura das pessoas.",
    "O Banco da Inglaterra foi fundado em 1694 para arrecadar fundos para a guerra da Inglaterra contra a França, tornando-se o protótipo dos bancos centrais.",
    "A Crise de 1873, conhecida como o 'Pânico de 1873', provocou o fechamento da Bolsa de Nova York por dez dias consecutivos.",
    "O padrão-ouro internacional foi formalmente estabelecido em 1821 pelo Reino Unido, vinculando a emissão de moeda diretamente às reservas de ouro.",
    "Nathan Rothschild lucrou fortemente com a Batalha de Waterloo em 1815 ao antecipar a vitória britânica por meio de pombos-correio e redes de espionagem.",
    // O Século XX e Grandes Crises
    "O Federal Reserve (Fed) foi criado em dezembro de 1913 através do Ato da Reserva Federal assinado pelo presidente Woodrow Wilson.",
    "A Grande Depressão foi despoletada pelo crash da bolsa de Nova York na 'Terça-Feira Negra', em 29 de outubro de 1929.",
    "O Acordo de Bretton Woods em 1944 atrelou as principais moedas globais ao dólar americano, que por sua vez era conversível em ouro.",
    "O presidente Richard Nixon suspendeu a convertibilidade do dólar em ouro em 1971, dando fim ao padrão-ouro global e inaugurando a era das moedas fiduciárias puras.",
    "O índice S&P 500 foi lançado oficialmente em março de 1957, transformando-se no principal termômetro do mercado acionário americano.",
    "A Black Monday de 19 de outubro de 1987 registrou a maior queda percentual diária da história do Dow Jones, de 22,6%.",
    "O economista Harry Markowitz revolucionou a gestão de investimentos em 1952 com a Teoria do Portfólio Moderno, introduzindo o conceito de diversificação.",
    // Inovação, Instrumentos e Tecnologia
    "O primeiro fundo de índice (ETF) moderno, o SPDR S&P 500 ETF (conhecido como SPY), foi lançado nos EUA em 1993.",
    "O NASDAQ, fundado em 1971, foi a primeira bolsa de valores totalmente eletrônica do mundo.",
    "Os contratos futuros de commodities padronizados ganharam tração institucional com a criação da Bolsa de Comércio de Chicago (CBOT) em 1848.",
    "O conceito de 'venda a descoberto' (short selling) foi documentado pela primeira vez na negociação de ações da VOC holandesa no século XVII.",
    "O modelo de precificação de opções Black-Scholes, publicado em 1973, fundamentou o mercado global de derivativos financeiros.",
    "O flash crash de 2010 apagou temporariamente cerca de 1 trilhão de dólares do mercado americano em minutos, impulsionado por algoritmos de alta frequência.",
    "O primeiro fundo mútuo moderno dos EUA, o Massachusetts Investors Trust, foi criado em março de 1924.",
    // Criptomoedas, Era Digital e Finanças Modernas
    "O Bitcoin (BTC) foi criado em janeiro de 2009 com a mineração do bloco gênesis por Satoshi Nakamoto.",
    "A primeira transação comercial real com Bitcoin ocorreu em 2010, quando 10.000 bitcoins foram trocados por duas pizzas.",
    "O termo Fintech surgiu na década de 1990 para descrever o uso de tecnologia emergente em serviços financeiros e bancários.",
    "O colapso do Lehman Brothers em setembro de 2008 desencadeou a pior crise financeira global desde a Grande Depressão.",
    "A bolha das empresas dot-com estourou em março de 2000, eliminando trilhões de dólares em valor de empresas de tecnologia na Nasdaq.",
    "O Banco Central do Brasil lançou o PIX em novembro de 2020, revolucionando os pagamentos instantâneos no país.",
    "As ordens de ações sem corretagem ganharam popularidade massiva na década de 2010 através de aplicativos de negociação de varejo como o Robinhood.",
    // Curiosidades Globais e Históricas
    "A Bolsa de Valores de Tóquio foi fundada em 1878, tornando-se o mercado financeiro mais influente da Ásia no pós-guerra.",
    "O termo 'Bull Market' (mercado em alta) e 'Bear Market' (mercado em baixa) pode derivar da forma como os animais atacam: o touro joga os chifres para cima, o urso empurra a pata para baixo.",
    "A primeira mulher a negociar na Bolsa de Nova York foi Victoria Woodhull, que abriu uma corretora junto com sua irmã em 1870.",
    "O 'Big Bang' de 1986 no Reino Unido desregulamentou o mercado financeiro britânico, digitalizando pregões e abolindo comissões fixas.",
    "A taxa de juros mais antiga registrada em contratos formais remonta aos tabletes de argila da Mesopotâmia por volta de 3000 a.C.",
    "O índice de preços ao consumidor (IPC) moderno começou a ser estruturado de forma estatística rigorosa durante a Primeira Guerra Mundial para calcular aumentos salariais.",
    "A Islândia nacionalizou seu sistema bancário por completo após o colapso financeiro de 2008, resultando na recuperação econômica posterior do país.",
    "O Mercado de Ações de Joanesburgo (JSE), na África do Sul, foi fundado em 1887 para atender ao boom da mineração de ouro e diamantes na região.",
    "As Letras do Tesouro (Treasury Bills) dos Estados Unidos são consideradas por muitos investidores globais como o ativo livre de risco padrão do sistema financeiro.",
    "O conceito de juros compostos foi chamado por Albert Einstein de 'a oitava maravilha do mundo'.",
];

async fn dashboard(user: User, repository: Repository) -> Result<Html<String>, AppError> {
    let user_assets = repository.get_user_assets(user.id()).await?;

    // Calculate total portfolio value using Rust iterators
    let total_carteira: f64 = user_assets.iter().map(|asset| asset.total_value()).sum();

    // Format in Rust to avoid template filter issues
    let total_carteira_formatado = format!("{:.2}", total_carteira).replace('.', ",");

    // Pick a random historical curiosity
    let mut rng = rand::rng();
    let curiosidade_sorteada = CURIOSIDADES_HISTORICAS
        .choose(&mut rng)
        .unwrap_or(&"A primeira ação negociada publicamente no mundo foi da Companhia Holandesa das Índias Orientais (VOC) em 1602.");

    let template = DashboardTemplate {
        user_name: user.username().clone(),
        assets: user_assets,
        total_carteira: total_carteira_formatado,
        curiosidade_historica: curiosidade_sorteada.to_string(),
    };

    Ok(Html(template.render()?))
}
